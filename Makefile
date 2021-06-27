export KDIR ?= /lib/modules/$(shell uname -r)/build

CLANG ?= clang
ifeq ($(origin CC),default)
CC := ${CLANG}
endif

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC) CONFIG_CC_IS_CLANG=y

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) CC=$(CC) clean
	rm -rf $(CURDIR)/target
	rm  $(CURDIR)/Cargo.lock

.PHONY: load
load:
	$(if $(strip $(shell lsmod | grep leftpad)), sudo rmmod leftpad)
	sudo insmod leftpad.ko
	$(eval MAJOR = $(shell cat /proc/devices | grep leftpad | cut -d' ' -f1))
	$(if $(wildcard /dev/leftpad), sudo rm /dev/leftpad)
	sudo mknod --mode=a=rw /dev/leftpad c $(MAJOR) 0

