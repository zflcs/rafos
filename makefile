run:
	make -C rafos-kernel run

disasm:
	make -C rafos-kernel disasm

test:
	make -C rafos-tests test

clean:
	rm -rf ./target