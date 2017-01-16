INSTALL_DIR=/usr/local/bin
ALL=wb_check wb_dump wb_meta wb_status

all: $(ALL)

clean:
	rm $(ALL)

install:
	install -m 555 $(ALL) $(INSTALL_DIR)

uninstall:
	rm $(INSTALL_DIR)/$(ALL)

%: %.go
	go build -o $@ $<
