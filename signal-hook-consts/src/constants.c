#define _POSIX_C_SOURCE 199309L
#include <stdio.h>
#include <signal.h>

int xy(const siginfo_t *x) {
	return x->si_uid;
}

/*
 * Unfortunately, some constants are not exported by libc. We cheat and
 * automatically get them from the C world.
 */
int main(int argc, const char *argv[]) {
#define C(NAME) printf("pub const " #NAME ": c_int = %d;\n", NAME)
	C(SI_USER);
	C(SI_QUEUE);
	return 0;
}
