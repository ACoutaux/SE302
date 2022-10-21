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

//Define the stacksize for the blink thread
#define STACKSIZE 1024

//Define the priority for the blink thread
#define PRIORITY 9

//Declare data structure for message queue
struct data_item_type {
	double dis_value;
};  

//Init message queue
K_MSGQ_DEFINE(my_msgq, sizeof(struct data_item_type), 10, 64); //10 items of 64 bits (double)

//Define device object for distance captor vl53l0X
const struct device *const sensor_dis = DEVICE_DT_GET_ANY(st_vl53l0x);

//Define led variable for led0
static struct gpio_dt_spec led = GPIO_DT_SPEC_GET(DT_ALIAS(led0), gpios);

void dis_sensor(void) {

	//Check if distance sensor divice is ready 
	if (!device_is_ready(sensor_dis))
	{
		printk("Error: distance sensor device is not ready");
			   
		return;
	}

	struct sensor_value distance;

	while(1) {
		sensor_sample_fetch(sensor_dis); //Reads data for sensor_dis channel
		sensor_channel_get(sensor_dis,SENSOR_CHAN_DISTANCE,&distance); //Stock values in distance structure
		double val = sensor_value_to_double(&distance); //Convert sensor value type to double
		k_msleep(50); //Waits 500s beetween each reading of sensor values
		while (k_msgq_put(&my_msgq, &val, K_NO_WAIT) != 0) {
            //If full clear and retry
            k_msgq_purge(&my_msgq);
        }
	}
}

void main(void)
{

	//Check if led is ready
	if (!device_is_ready(led.port))
	{
		printk("Error: led device is not ready");
		return; 
	}

	double data; //Contains data received by message queue

	while(1) {
		//The led is set to sleep every data*100 ms (taking in account that data are in centimeters)
		k_msgq_get(&my_msgq, &data, K_FOREVER);
		gpio_pin_set_dt(&led,1);
		k_msleep(data*100);
		gpio_pin_set_dt(&led,0);
		k_msleep(data*100);
	}
}

//Define and start at compilation distance sensor thread
K_THREAD_DEFINE(dis_sensor_id, STACKSIZE, dis_sensor, NULL, NULL, NULL,PRIORITY, 0, 0);
