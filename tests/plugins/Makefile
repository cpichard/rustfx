
# Only works on MacOS for now

OFX/basic.ofx.bundle/Contents/MacOS-x86-64/basic.ofx: basic.o
	g++ -bundle basic.o -o basic.ofx
	mkdir -p OFX/basic.ofx.bundle/Contents/MacOS-x86-64/
	mv basic.ofx $@ 

basic.o: basic.cpp ofxUtilities.hpp
	g++ -c -g -fvisibility=hidden -fPIC -I../../src/ofx basic.cpp -o basic.o

clean: 
	rm *.o
