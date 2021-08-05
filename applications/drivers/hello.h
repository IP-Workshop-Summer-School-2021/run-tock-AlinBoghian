#pragma once
#include "tock.h"
const unsigned int HELLO_DRIVER_NUM = 0xa0000;


bool hello_is_present (void);
bool hello_print(void);
bool hello_up(void);
bool hello_down(void);
