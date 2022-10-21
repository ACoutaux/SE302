/*
 * Copyright (c) 2016 Open-RnD Sp. z o.o.
 * Copyright (c) 2020 Nordic Semiconductor ASA
 *
 * SPDX-License-Identifier: Apache-2.0
 */

#include <zephyr/kernel.h>
#include <zephyr/device.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/sys/util.h>
#include <zephyr/sys/printk.h>
#include <inttypes.h>
#include <zephyr/drivers/led.h>
#include <zephyr/drivers/sensor.h>
#include <zephyr/drivers/i2c.h>

//Define device object for distance captor vl53l0X
const struct device *const sensor_dis = DEVICE_DT_GET_ANY(st_vl53l0x);

//Define led structure for led0
static struct gpio_dt_spec led = GPIO_DT_SPEC_GET(DT_ALIAS(led0), gpios);

//Define bus_i2c structure for bus i2c
static struct i2c_dt_spec bus_i2c = I2C_DT_SPEC_GET(DT_ALIAS(accel0));

void main(void)
{

	//Check if led is ready
	if (!device_is_ready(led.port))
	{
		printk("Error: led device is not ready");
		return; 
	}

	//Check if accelerometer is ready
	if (!device_is_ready(bus_i2c.bus))
	{
		printk("Error: i2c bus device is not ready");
		return; 
	}

	while(1) {	
		uint8_t who_am_i; //variable for WHO_AM_I register
		i2c_reg_read_byte_dt(&bus_i2c,0x0f, &who_am_i);	//reads content of regiser at adress 0f
		printk("who_am_i: %d\n",who_am_i); //print register content in decimal format
		k_msleep(1000);
	}
}
