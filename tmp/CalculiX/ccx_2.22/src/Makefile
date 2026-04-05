
CFLAGS = -Wall -O2  -I ../../ -DARCH="Linux" -DSPOOLES -DARPACK -DMATRIXSTORAGE -DNETWORKOUT
FFLAGS = -Wall -O2

CC=cc
FC=gfortran

.c.o :
	$(CC) $(CFLAGS) -c $<
.f.o :
	$(FC) $(FFLAGS) -c $<

include Makefile.inc

SCCXMAIN = ccx_2.22.c

OCCXF = $(SCCXF:.f=.o)
OCCXC = $(SCCXC:.c=.o)
OCCXMAIN = $(SCCXMAIN:.c=.o)

DIR=../../../SPOOLES.2.2

LIBS = \
       ../../spooles.a \
	/opt/homebrew/lib/libarpack.dylib

ccx_2.22: $(OCCXMAIN) ccx_2.22.a  $(LIBS)
	./date.pl; $(CC) $(CFLAGS) -c ccx_2.22.c; $(FC)  -Wall -O2 -o $@ $(OCCXMAIN) ccx_2.22.a $(LIBS) -fopenmp -lm -lc /opt/homebrew/Cellar/openblas/0.3.32/lib/libopenblas.dylib

ccx_2.22.a: $(OCCXF) $(OCCXC)
	ar vr $@ $?
