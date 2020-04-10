.PHONY: install uninstall
install:
	mkdir -p /usr/share/sockgit/templates && cp -r templates/* /usr/share/sockgit/templates/
	cp sockgit-listen* /etc/systemd/system/
	ln -s $(shell pwd)/sockgit.sh /usr/bin/sockgit

uninstall:
	rm -f /etc/systemd/system/sockgit-listen.socket
	rm -f /etc/systemd/system/sockgit-listen@.service
	rm -rf /usr/share/sockgit
	rm -f /usr/bin/sockgit
