FUNCTIONS := handler
STACK_NAME := geoip
ARCH := aarch64-unknown-linux-gnu
ARCH_SPLIT = $(subst -, ,$(ARCH))

build:
ifeq ("$(shell zig targets | jq -r .native.cpu.arch)-$(shell zig targets | jq -r .native.os)-$(shell zig targets | jq -r .native.abi)", "$(word 1,$(ARCH_SPLIT))-$(word 3,$(ARCH_SPLIT))-$(word 4,$(ARCH_SPLIT))")
	@echo "Same host and target. Using native build"
	cargo build --release --target $(ARCH)
else
	@echo "Different host and target. Using zigbuild"
	cargo zigbuild --release --target $(ARCH)
endif

	rm -rf ./build
	mkdir -p ./build
	${MAKE} ${MAKEOPTS} $(foreach function,${FUNCTIONS}, build-${function})

build-%:
	mkdir -p ./build/$*
	cp -v ./target/$(ARCH)/release/$* ./build/$*/bootstrap

deploy:
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --stack-name ${STACK_NAME}-s3 --template-file ./deployment/s3.yml
	sam deploy --guided --no-fail-on-empty-changeset --no-confirm-changeset --stack-name ${STACK_NAME}-geoip-api --template-file ./deployment/api.yml
delete:
	sam delete --stack-name ${STACK_NAME}-api
	sam delete --stack-name ${STACK_NAME}-s3