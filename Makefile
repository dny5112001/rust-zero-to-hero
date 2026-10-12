# Build the Rust app and deploy it to a local Kubernetes cluster with Terraform.
#   make deploy   -> everything (cluster, ingress, database, app)
#   make test     -> smoke test through the ingress
#   make destroy  -> remove everything

TF        := terraform
TFDIR     := deploy/terraform
KUBECONFIG := $(CURDIR)/$(TFDIR)/01-cluster/kubeconfig
CLUSTER   := rust
# Tag = hash of the source, so every code change gets a new tag -> rolling update.
TAG       := $(shell cat notes-api/Cargo.toml notes-api/Cargo.lock notes-api/Dockerfile $$(find notes-api/src notes-api/migrations -type f | sort) | shasum | cut -c1-10)
IMAGE     := notes-api:$(TAG)
URL       := http://api.localhost:8070

export KUBECONFIG

.PHONY: deploy cluster platform image app test status destroy

deploy: cluster platform app test

cluster:
	$(TF) -chdir=$(TFDIR)/01-cluster init -input=false
	$(TF) -chdir=$(TFDIR)/01-cluster apply -auto-approve

platform:
	$(TF) -chdir=$(TFDIR)/02-platform init -input=false
	$(TF) -chdir=$(TFDIR)/02-platform apply -auto-approve

image: ## build the Docker image and copy it into the kind nodes (no registry needed)
	docker build -t $(IMAGE) notes-api
	kind load docker-image $(IMAGE) --name $(CLUSTER)

app: image
	$(TF) -chdir=$(TFDIR)/03-app init -input=false
	$(TF) -chdir=$(TFDIR)/03-app apply -auto-approve -var image=$(IMAGE)
	kubectl -n notes rollout status deploy/notes-api --timeout=180s

test:
	curl -fsS --retry 15 --retry-all-errors --retry-delay 1 $(URL)/health && echo
	-curl -fsS -X POST $(URL)/auth/register -H 'content-type: application/json' -d '{"email":"smoke@example.com","password":"a-long-password"}' && echo
	@TOKEN=$$(curl -fsS -X POST $(URL)/auth/login -H 'content-type: application/json' -d '{"email":"smoke@example.com","password":"a-long-password"}' | sed -E 's/.*"access_token":"([^"]+)".*/\1/'); \
	curl -fsS -X POST $(URL)/notes -H "authorization: Bearer $$TOKEN" -H 'content-type: application/json' -d '{"title":"Hello from Kubernetes"}' && echo; \
	curl -fsS $(URL)/notes -H "authorization: Bearer $$TOKEN" && echo

status:
	kubectl get nodes
	kubectl -n notes get pods,svc,ingress,pvc

destroy:
	-$(TF) -chdir=$(TFDIR)/03-app destroy -auto-approve -var image=$(IMAGE)
	-$(TF) -chdir=$(TFDIR)/02-platform destroy -auto-approve
	$(TF) -chdir=$(TFDIR)/01-cluster destroy -auto-approve
