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
#include <math.h>

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

	i2c_reg_write_byte_dt(&bus_i2c, 0x15, 0b00010000); //set XL_HM_MODE to 1 to disable high performance mode
	i2c_reg_write_byte_dt(&bus_i2c, 0x10, 0b10110000); //set ODR_XLR[3:0] to 1 0 1 1 for a frequency at 1.6 Hz

	while(1) {	
		uint8_t acc_x1,acc_x2,acc_y1,acc_y2,acc_z1,acc_z2; //variables for accelerometer measurements
		uint16_t acc_x,acc_y,acc_z,norm_acc;

		//Read linear acceleraion for X axis
		i2c_reg_read_byte_dt(&bus_i2c,0x28, &acc_x1);	
		i2c_reg_read_byte_dt(&bus_i2c,0x29, &acc_x2);

		//Read linear acceleraion for Y axis
		i2c_reg_read_byte_dt(&bus_i2c,0x2a, &acc_y1);	
		i2c_reg_read_byte_dt(&bus_i2c,0x2b, &acc_y2);

		//Read linear acceleraion for Z axis
		i2c_reg_read_byte_dt(&bus_i2c,0x2c, &acc_z1);	
		i2c_reg_read_byte_dt(&bus_i2c,0x2d, &acc_z2);

		//Compute linear accelerations and norm
		acc_x = (acc_x2 << 8) + acc_x1;
		acc_y = (acc_y2 << 8) + acc_y1;
		acc_z = (acc_z2 << 8) + acc_z1;
		norm_acc = sqrt((acc_x1^2) + (acc_y^2) + (acc_z^2));

		printk("Acceleration X: %d Acceleration Y: %d Acceleration Z: %d\n",acc_x,acc_y,acc_z); //print register content in decimal format
		printk("Norme: %d\n", norm_acc);
		k_msleep(500); //sleeps for 500 ms
	}
}
