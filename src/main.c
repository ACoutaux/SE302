#include <stdio.h>
#include <zephyr/kernel.h>
#include <zephyr/device.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/led.h>

//Get a pointer on the led driver device using its path in the device tree (/leds)
const struct device *const led_dev = DEVICE_DT_GET(DT_PATH(leds)); 

int main() {
  
  //Make the led 0 and 1 blink
  for (;;) {
    led_on(led_dev,1);
    led_on(led_dev,0);
    led_on(led_dev,2);
    
    k_sleep(K_SECONDS(1));
    
    led_off(led_dev,1);
    led_off(led_dev,0);
    led_off(led_dev,2);
    k_sleep(K_SECONDS(1));
  }
  return 0;
}