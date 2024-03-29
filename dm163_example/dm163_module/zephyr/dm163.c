// Enter the driver name so that when we initialize the (maybe numerous)
// DM163 peripherals we can designated them by index.
#define DT_DRV_COMPAT siti_dm163

#include <math.h>
#include <stdio.h>
#include <zephyr/device.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/led.h>
#include <zephyr/kernel.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(dm163, LOG_LEVEL_DBG);

struct dm163_config
{
  const struct gpio_dt_spec en;
  const struct gpio_dt_spec gck;
  const struct gpio_dt_spec lat;
  const struct gpio_dt_spec rst;
  const struct gpio_dt_spec selbk;
  const struct gpio_dt_spec sin;
};

#define NUM_LEDS 8
#define NUM_CHANNELS (NUM_LEDS * 3)

struct dm163_data
{
  uint8_t brightness[NUM_CHANNELS];
  uint8_t channels[NUM_CHANNELS];
  struct k_mutex dm163_mutex;
};



#define CONFIGURE_PIN(dt, flags)                           \
  do                                                       \
  {                                                        \
    if (!device_is_ready((dt)->port))                      \
    {                                                      \
      LOG_ERR("device %s is not ready", (dt)->port->name); \
      return -ENODEV;                                      \
    }                                                      \
    gpio_pin_configure_dt(dt, flags);                      \
  } while (0)

// Declare implemented functions
static int dm163_on(const struct device *dev, uint32_t led);
static int dm163_off(const struct device *dev, uint32_t led);
static int dm163_set_brightness(const struct device *dev, uint32_t led, uint8_t value);
static void flush_channels(const struct device *dev);
static void flush_brightness(const struct device *dev);
int dm163_set_color(const struct device *dev, uint32_t led, uint8_t num_colors, const uint8_t *color);
int dm163_write_channels(const struct device *dev, uint32_t start_channel, uint32_t num_channels, const uint8_t *buf);

static int dm163_init(const struct device *dev)
{
  const struct dm163_config *config = dev->config;

  struct dm163_data *data = dev->data; // define data and retrieve it from dev device

  LOG_DBG("starting initialization of device %s", dev->name);

  // Disable DM163 outputs while configuring if this pin
  // is connected.
  if (config->en.port)
  {
    CONFIGURE_PIN(&config->en, GPIO_OUTPUT_INACTIVE);
  }
  // Configure all pins. Make reset active so that the DM163
  // initiates a reset. We want the clock (gck) and latch (lat)
  // to be inactive at start. selbk will select bank 1 by default.
  CONFIGURE_PIN(&config->rst, GPIO_OUTPUT_ACTIVE);
  CONFIGURE_PIN(&config->gck, GPIO_OUTPUT_INACTIVE);
  CONFIGURE_PIN(&config->lat, GPIO_OUTPUT_INACTIVE);
  CONFIGURE_PIN(&config->selbk, GPIO_OUTPUT_ACTIVE);
  CONFIGURE_PIN(&config->sin, GPIO_OUTPUT);
  k_usleep(1); // 100ns min
  // Cancel reset by making it inactive.
  gpio_pin_set_dt(&config->rst, 0);

  // Init data and send it to DM163
  memset(&data->brightness, 0x3f, sizeof(data->brightness));
  memset(&data->channels, 0x00, sizeof(data->channels));
  flush_brightness(dev);
  flush_channels(dev);

  // Enable the outputs if this pin is connected.
  if (config->en.port)
  {
    gpio_pin_set_dt(&config->en, 1);
  }
  LOG_INF("device %s initialized", dev->name);

  k_mutex_init(&data->dm163_mutex);

  return 0;
}

static const struct led_driver_api dm163_api = {
    .on = dm163_on,
    .off = dm163_off,
    .set_brightness = dm163_set_brightness,
    .set_color = dm163_set_color,
    .write_channels = dm163_write_channels,
};

// Macro to initialize the DM163 peripheral with index i
#define DM163_DEVICE(i)                                                        \
                                                                               \
  /* Build a dm163_config for DM163 peripheral with index i, named          */ \
  /* dm163_config_/i/ (for example dm163_config_0 for the first peripheral) */ \
  static const struct dm163_config dm163_config_##i = {                        \
      .en = GPIO_DT_SPEC_GET_OR(DT_DRV_INST(i), en_gpios, {0}),                \
      .gck = GPIO_DT_SPEC_GET(DT_DRV_INST(i), gck_gpios),                      \
      .lat = GPIO_DT_SPEC_GET(DT_DRV_INST(i), lat_gpios),                      \
      .rst = GPIO_DT_SPEC_GET(DT_DRV_INST(i), rst_gpios),                      \
      .selbk = GPIO_DT_SPEC_GET(DT_DRV_INST(i), selbk_gpios),                  \
      .sin = GPIO_DT_SPEC_GET(DT_DRV_INST(i), sin_gpios),                      \
  };                                                                           \
                                                                               \
  /* Build a new dm163_data_/i/ structure for dynamic data                  */ \
  static struct dm163_data dm163_data_##i = {};                                \
                                                                               \
  DEVICE_DT_INST_DEFINE(i, &dm163_init, NULL, &dm163_data_##i,                 \
                        &dm163_config_##i, POST_KERNEL,                        \
                        CONFIG_LED_INIT_PRIORITY, &dm163_api);

// Apply the DM163_DEVICE to all DM163 peripherals not marked "disabled"
// in the device tree and pass it the corresponding index.
DT_INST_FOREACH_STATUS_OKAY(DM163_DEVICE)

// Send (bits) bits of data on the DM163
static void pulse_data(const struct dm163_config *config, uint8_t data, int bits)
{
  for (int i = bits - 1; i >= 0; i--)
  {
    gpio_pin_set_dt(&config->sin, (bool)(data & (1 << (i))));
    gpio_pin_set_dt(&config->gck, 1); // config->gck active then inactive (clock pulse)
    gpio_pin_set_dt(&config->gck, 0);
  }
}

// Send all channel information to bank 1 in reverse order
static void flush_channels(const struct device *dev)
{
  const struct dm163_config *config = dev->config;
  struct dm163_data *data = dev->data;

  gpio_pin_set_dt(&config->selbk, 1); // selection of bank 1 (should be value by default)

  for (int i = NUM_CHANNELS - 1; i >= 0; i--)
  {
    pulse_data(config, data->channels[i], 8);
  }
  gpio_pin_set_dt(&config->lat, 1);
  gpio_pin_set_dt(&config->lat, 0);
}

// Send brightness information to bank 0
static void flush_brightness(const struct device *dev)
{

  const struct dm163_config *config = dev->config; // retrieve config from dev
  struct dm163_data *data = dev->data;             // retrieve data from dev

  gpio_pin_set_dt(&config->selbk, 0); // selection of bank 0

  for (int i = NUM_CHANNELS - 1; i >= 0; i--)
  {
    pulse_data(config, data->brightness[i], 6);
  }
  gpio_pin_set_dt(&config->lat, 1);
  gpio_pin_set_dt(&config->lat, 0);
  gpio_pin_set_dt(&config->selbk, 1); // reselection of bank 1
}

// Send value brightness to led
static int dm163_set_brightness(const struct device *dev, uint32_t led, uint8_t value)
{
  if (led >= NUM_LEDS || led < 0)
  {
    return -EINVAL;
  }
  struct dm163_data *data = dev->data;             // retrieve data from dev
  int led_brightness = (uint16_t)value * 63 / 100; // convert brightness from 0-100 to 0-63

  // Set brigthness into the corresponding channels of the led
  data->brightness[led * 3] = led_brightness;
  data->brightness[(led * 3) + 1] = led_brightness;
  data->brightness[(led * 3) + 2] = led_brightness;

  k_mutex_lock(&data->dm163_mutex,K_FOREVER);
  flush_brightness(dev); // send brigthness
  k_mutex_unlock(&data->dm163_mutex);
  return 0;
}

static int dm163_on(const struct device *dev, uint32_t led)
{
  struct dm163_data *data = dev->data; // retrieve data from dev

  // Set colors to the corresponding channels of the led
  data->channels[led * 3] = 0xff;
  data->channels[(led * 3) + 1] = 0xff;
  data->channels[(led * 3) + 2] = 0xff;

  k_mutex_lock(&data->dm163_mutex,K_FOREVER);
  flush_channels(dev); // send data
  k_mutex_unlock(&data->dm163_mutex);
  return 0;
}

static int dm163_off(const struct device *dev, uint32_t led)
{
  struct dm163_data *data = dev->data; // retrieve data from dev

  // Set colors to the corresponding channels of the led
  data->channels[led * 3] = 0x0;
  data->channels[(led * 3) + 1] = 0x0;
  data->channels[(led * 3) + 2] = 0x0;
  
  k_mutex_lock(&data->dm163_mutex,K_FOREVER);
  flush_channels(dev); // send data
  k_mutex_unlock(&data->dm163_mutex);
  return 0;
}

//Set led on with specified color (0 by default)
int dm163_set_color(const struct device *dev, uint32_t led, uint8_t num_colors, const uint8_t *color)
{
  if (led >= NUM_LEDS || led < 0 || num_colors > 3)
    return -EINVAL;
  
  struct dm163_data *data = dev->data; // retrieve data from dev

  if (num_colors == 0) {
    data->channels[led * 3] = 0x0;
    data->channels[(led * 3) + 1] = 0x0;
    data->channels[(led * 3) + 2] = 0x0;
  } else if (num_colors == 1) {
    data->channels[led * 3] = color[0];
    data->channels[(led * 3) + 1] = 0x0;
    data->channels[(led * 3) + 2] = 0x0;
  } else if (num_colors == 2) {
    data->channels[led * 3] = color[0];
    data->channels[(led * 3) + 1] = color[1];
    data->channels[(led * 3) + 2] = 0x0;
  } else {
    data->channels[led * 3] = color[0];
    data->channels[(led * 3) + 1] = color[1];
    data->channels[(led * 3) + 2] = color[2];
  }

  k_mutex_lock(&data->dm163_mutex,K_FOREVER);
  flush_channels(dev); //send color data
  k_mutex_unlock(&data->dm163_mutex);
  return 0;
}

//Set channels in bulk
int dm163_write_channels(const struct device *dev, uint32_t start_channel, uint32_t num_channels, const uint8_t *buf) {
   
  if (start_channel >= NUM_CHANNELS * 8 || start_channel < 0 || num_channels >= NUM_CHANNELS * 8 || num_channels < 0)
    return -EINVAL;

  struct dm163_data *data = dev->data; //retrieve data from dev 
  for (int i=start_channel; i<num_channels+start_channel; i++) {
    data->channels[i] = buf[i];
  }

  flush_channels(dev); //send channels data

  return 0;
}
