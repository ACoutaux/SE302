#include <zephyr/device.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/led.h>
#include <zephyr/kernel.h>
#include <zephyr/sys/printk.h>
#include <stdio.h>
#include <stdlib.h>

// Define the stacksize for the complementary filter thread
#define STACKSIZE 1024

// Define the priority for the complementary filter thread
#define PRIORITY 9

// Define and init semaphore (counter of 1 maximum)
K_SEM_DEFINE(disp_sem, 0, 1);

// Declare function for disp_next_line_id thread
void disp_next_line(void);

#define DM163_NODE DT_NODELABEL(dm163)
static const struct device *dm163_dev = DEVICE_DT_GET(DM163_NODE);

#define RGB_MATRIX_NODE DT_NODELABEL(rgb_matrix)
BUILD_ASSERT(DT_PROP_LEN(RGB_MATRIX_NODE, rows_gpios) == 8);

// Define image structure
uint8_t image[192];

static const struct gpio_dt_spec rows[] = {
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 0),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 1),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 2),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 3),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 4),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 5),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 6),
    GPIO_DT_SPEC_GET_BY_IDX(RGB_MATRIX_NODE, rows_gpios, 7),
};

// Function of thread disp_next_line_id
void disp_next_line(void)
{
  while (1)
  {
    for (int row = 0; row < 8; row++)
    {
      gpio_pin_set_dt(&rows[row], 1);
      led_write_channels(dm163_dev, row, 3 * 8, &image[row * 3 * 8]);
      k_sem_take(&disp_sem, K_FOREVER); //take semaphore beetween each line writing
      gpio_pin_set_dt(&rows[row], 0);
    }
  }
}

int main()
{
  // const uint8_t colors[] = {0x0,0X0,0xff}; //set led color to blue
  if (!device_is_ready(dm163_dev))
  {
    return -ENODEV;
  }
  for (int row = 0; row < 8; row++)
    gpio_pin_configure_dt(&rows[row], GPIO_OUTPUT_INACTIVE);

  // Set brightness to 5% for all leds so that we don't become blind
  for (int i = 0; i < 8; i++)
    led_set_brightness(dm163_dev, i, 5);

  // Remplissage de Image (impossible d'initialiser le tableau normalement (?))
  for (int i = 0; i < 192; i++)
  {
    image[i] = 0xff;
  }

  // Loop for timeed release of semaphore
  for (;;)
  {
    k_msleep(2); // waits 1/8*60 seconds
    k_sem_give(&disp_sem); //Release semaphore
  }

  return 0;
}

// Define thread for next line display
K_THREAD_DEFINE(disp_next_line_id, STACKSIZE, disp_next_line, NULL, NULL, NULL, PRIORITY, 0, 0);
