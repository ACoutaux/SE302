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

//Define device object for distance captor vl53l0X
const struct device *const sensor_dis = DEVICE_DT_GET_ANY(st_vl53l0x);

void main(void)
{

	struct sensor_value distance; 

	//Check if distance sensor divice is ready 
	if (!device_is_ready(sensor_dis))
	{
		printk("Error: distance sensor device is not ready");
			   
		return;
	}

	while(1) {
		sensor_sample_fetch(sensor_dis); //Reads data for sensor_dis channel
		sensor_channel_get(sensor_dis,SENSOR_CHAN_DISTANCE,&distance); //Stock values in distance structure
		double val = sensor_value_to_double(&distance); //Convert sensor value type to double
		printk("distance: %lf\n", val);

		k_msleep(100); //Waits 100 ms beetween each reading of sensor values
	}
}
