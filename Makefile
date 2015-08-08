CXX = g++
LIBS =
LDFLAGS = -flto
NODEPS = -std=c++11 -Wall -Wextra -Werror -g -I. -gdwarf-3
# Use -MMD to generate dependencies
CFLAGS = $(NODEPS) -MMD
ODIR = build
SRCS = $(shell find src -name '*.cpp' -not -name '.\#*')
DIRS = $(shell find src -type d | sed s/src/$(ODIR)/)
OBJS = $(SRCS:src/%.cpp=$(ODIR)/%.o)
DEPS = $(SRCS:src/%.cpp=$(ODIR)/%.d)
BIN = pecan

$(shell mkdir -p $(DIRS))

all: $(BIN)
$(BIN): $(OBJS)
	$(CXX) -o $(BIN) $(OBJS) $(LIBS) $(CFLAGS) $(LDFLAGS)

$(ODIR)/%.o: src/%.cpp
	$(CXX) -o $@ $< -c $(CFLAGS)

clean:
	rm -f $(OBJS)
	rm -f $(DEPS)
	rm -f $(BIN)
	rmdir -p $(DIRS)

# Include generated dependencies for smarter rebuilds
-include $(DEPS)
