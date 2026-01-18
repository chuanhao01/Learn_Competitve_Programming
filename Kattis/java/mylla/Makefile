# Help Commands
.PHONY: default
default: help

# Main Commands
.PHONY: cc
cc:
	javac -encoding UTF-8 Main.java

.PHONY: c
c:
	javac Main.java

.PHONY: r
r:
	java Main

.PHONY: rr
rr:
	java -Dfile.encoding=UTF-8 -XX:+UseSerialGC -Xss64m Main

.PHONY: all
all: cc rr

.PHONY: clean
clean:
	rm -rf *.class


.PHONY: help
help:
	@echo 'Usage: make [command]'
	@echo 'c:     - javac/Compiles all java file'
	@echo 'cc:    - javac/Compiles like kattis'
	@echo 'c      - Runs Main'
	@echo 'rr:    - Runs like kattis'
	@echo 'all:   - Runs cc and rr'
	@echo 'clean: - rm -rf *.class'
