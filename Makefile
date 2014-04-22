RUSTC := rustc
RUSTDOC := rustdoc
RUSTFLAGS := -O
BUILDDIR := build
EXAMPLE_DIR := examples
DOC_DIR := doc


all: termbox doc examples

termbox:
	mkdir -p $(BUILDDIR)
	$(RUSTC) $(RUSTFLAGS) src/termbox/lib.rs --out-dir=$(BUILDDIR)

examples: termbox
	mkdir -p $(EXAMPLE_DIR)
	$(RUSTC) $(RUSTFLAGS) -L $(BUILDDIR) src/examples/test1.rs --out-dir=$(EXAMPLE_DIR)

clean:
	rm -f $(BUILDDIR)/*.so
	rm -f $(BUILDDIR)/*.rlib
	rm -rf doc
	rm -rf $(EXAMPLE_DIR)

doc: termbox
	$(RUSTDOC) -L $(BUILDDIR) src/termbox/lib.rs -o $(DOC_DIR)

.PHONY: clean doc examples termbox
