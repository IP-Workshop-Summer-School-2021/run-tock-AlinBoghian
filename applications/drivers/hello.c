#include "hello.h"

#include "tock.h"

bool hello_is_present(void){
	syscall_return_t res = command (HELLO_DRIVER_NUM,0,0,0);
	return (res.type ==TOCK_SYSCALL_SUCCESS);
}

bool hello_print(void){
	syscall_return_t res = command(HELLO_DRIVER_NUM,1,0,0);
	return (res.type ==TOCK_SYSCALL_SUCCESS);
}
bool hello_up(void){
	syscall_return_t res = command(HELLO_DRIVER_NUM,2,0,0);
	return (res.type ==TOCK_SYSCALL_SUCCESS);
}
bool hello_down(void){
	syscall_return_t res = command(HELLO_DRIVER_NUM,3,0,0);
	return (res.type ==TOCK_SYSCALL_SUCCESS);
}

