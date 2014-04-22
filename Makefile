RUSTC := rustc
RUSTDOC := rustdoc
RUSTFLAGS := -O
BUILDDIR := build

all: termbox doc examples

termbox:
	mkdir -p $(BUILDDIR)
	$(RUSTC) $(RUSTFLAGS) src/termbox/lib.rs --out-dir=$(BUILDDIR)

examples:
	$(RUSTC) $(RUSTFLAGS) -L $(BUILDDIR) examples/test1.rs --out-dir=$(BUILDDIR)

clean:
	rm -f $(BUILDDIR)/*.so
	rm -f $(BUILDDIR)/*.rlib
	rm -f $(BUILDDIR)/test1
	rm -rf doc

doc:
	$(RUSTDOC) src/termbox/lib.rs

.PHONY: clean doc examples