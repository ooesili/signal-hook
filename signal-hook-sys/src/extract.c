#include <signal.h>
#include <stdint.h>

const uint8_t ORIGIN_UNKNOWN = 0;
const uint8_t ORIGIN_PROCESS = 1;
const uint8_t ORIGIN_KERNEL = 2;

uint8_t sighook_signal_origin(const siginfo_t *info, pid_t *pid, uid_t *uid) {
	switch (info->si_code) {
		case SI_USER:
		case SI_QUEUE:
		case SI_MESGQ:
			*pid = info->si_pid;
			*uid = info->si_uid;
			return ORIGIN_PROCESS;
#ifdef SI_KERNEL
		case SI_KERNEL:
			return ORIGIN_KERNEL;
#endif
		default:
			return ORIGIN_UNKNOWN;
	}
}
