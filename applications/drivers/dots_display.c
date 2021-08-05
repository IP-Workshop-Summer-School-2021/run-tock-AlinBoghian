#include "tock.h"
#include "hello.h"

void display(char c){
    syscall_return_t res = command (DISPLAY_DRIVER_NUM,1,c,0);
	return (res.type ==TOCK_SYSCALL_SUCCESS);
}