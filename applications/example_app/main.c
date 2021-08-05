/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include "example_driver.h"

int main(void) {
  //printf ("Hello World!\r\n");
  //example_driver_action ();
  if(hello_is_present()){
      printf("Hello driver is present\n");
      hello_print();
      hello_up();
      hello_up();
      hello_down();
      hello_print();
   }
   else
      printf("Hello driver isn't present\n");
  return 0;
}
