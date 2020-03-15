.PHONY: pbacklight install clean

pbacklight:
	@make -C "$(CURDIR)/pbacklight" release

install: pbacklight
	install -m0644 "$(CURDIR)/systemd/pbp-brightness.service" "/usr/lib/systemd/system/pbp-brightness.service"
	systemctl daemon-reload
	install -m0755 "$(CURDIR)/pbacklight/target/release/pbacklight" "/usr/bin/pbacklight"

clean:
	@make -C "$(CURDIR)/pbacklight" clean
