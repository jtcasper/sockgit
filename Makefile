.PHONY: install uninstall
install:
	cp sockgit-listen* /etc/systemd/system/
	ln -s $(shell pwd)/sockgit.sh /usr/bin/sockgit

uninstall:
	rm -f /etc/systemd/system/sockgit-listen.socket
	rm -f /etc/systemd/system/sockgit-listen@.service
	rm -f /usr/bin/sockgit
