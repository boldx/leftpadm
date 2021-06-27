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

.PHONY: load unload
unload:
	$(if $(strip $(shell lsmod | grep leftpad)), sudo rmmod leftpad)
	$(if $(wildcard /dev/leftpad), sudo rm /dev/leftpad)

load: unload
	sudo insmod leftpad.ko
	sudo mknod --mode=a=rw /dev/leftpad c $$(cat /proc/devices | grep leftpad | cut -d' ' -f1) 0

