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
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define PI 3.14159265

// Define the stacksize for the complementary filter thread
#define STACKSIZE 1024

// Define the priority for the complementary filter thread
#define PRIORITY 9

K_MUTEX_DEFINE(gyro_acc_mutex); // define and init mutex for acceleromter and gyroscope datas

// Structure for k_work object to read and display accelerometer value
struct k_work read_display_accelerometer_work;

// Structure for acceleromter measurements in callback functions
static struct gpio_callback accelerometer_data;

// Define device object for distance captor vl53l0X
const struct device *const sensor_dis = DEVICE_DT_GET_ANY(st_vl53l0x);

// Define acceleromter structure for LSM6DSL of bus i2c
static struct i2c_dt_spec accelerometer = I2C_DT_SPEC_GET(DT_ALIAS(accel0));

// Define mC irq sructure for accelerometer
static const struct gpio_dt_spec accelerometer_irq = GPIO_DT_SPEC_GET(DT_ALIAS(accel0), irq_gpios);

// Variables for accelerometer measurements
static uint8_t acc_x1, acc_x2, acc_y1, acc_y2, acc_z1, acc_z2, status, gyro_x1, gyro_x2, gyro_y1, gyro_y2, gyro_z1, gyro_z2;
int16_t acc_x, acc_y, acc_z, gyro_x, gyro_y, gyro_z;
double teta_x, teta_y, teta_speed_x, teta_speed_y, teta_speed_z, Tilt_X, Tilt_Y;
static double tilt_angle_x, tilt_angle_y = 0.0; // static variable for gyroscope measurement to save previous value beetween each measurements

//Entry point function of filter_id thread
void complementary_filter(void) {
	while (1) {
		k_msleep(200); //waits 200 ms between each display
		k_mutex_lock(&gyro_acc_mutex, K_FOREVER); //lock mutex to compute and display data

		// Compute linear accelerations and gyroscope measurements
		acc_x = ((acc_x2 << 8) + acc_x1);
		acc_y = ((acc_y2 << 8) + acc_y1);
		acc_z = ((acc_z2 << 8) + acc_z1);
		gyro_x = ((gyro_x2 << 8) + gyro_x1);
		gyro_y = ((gyro_y2 << 8) + gyro_y1);
		gyro_z = ((gyro_z2 << 8) + gyro_z1);

		// Compute angular positions
		teta_x = atan(acc_x * 0.00059857177 / sqrt((pow(acc_y * 0.00059857177, 2) + pow(acc_z * 0.00059857177, 2)))) * 180 / PI; // converted in degrees
		teta_y = atan(acc_y * 0.00059857177 / sqrt((pow(acc_x * 0.00059857177, 2) + pow(acc_z * 0.00059857177, 2)))) * 180 / PI;
		teta_speed_x = gyro_x * 0.00762939453; // converted in dps
		teta_speed_y = gyro_y * 0.00762939453;
		teta_speed_z = gyro_z * 0.00762939453;
		tilt_angle_x = tilt_angle_x + (teta_speed_x * 0.00961538461); // measurements rate set to 104 Hz for gyroscope
		tilt_angle_y = tilt_angle_y + (teta_speed_y * 0.00961538461);

		//Directly print with complementary filter 95% accelerometer and 5% gyroscope
		printk("TiltX: %f TiltY: %f\n",(0.95 * teta_x) + (0.05 * (-tilt_angle_y*100.0)),(0.9 * teta_y) + (0.1 * tilt_angle_x * 100.0));

		k_mutex_unlock(&gyro_acc_mutex); //unlock mutex to give it back to read_display_accelerometer work
	}
}

void read_display_accelerometer(struct k_work *item)
{
	// Read STATUS_REG register
	i2c_reg_read_byte_dt(&accelerometer, 0x1e, &status);

	if (status >= 0x3)
	{ // data available for both gyroscope and accelerometer

		k_mutex_lock(&gyro_acc_mutex, K_FOREVER); // lock mutex before reading datas

		// Read gyroscope data for X axis
		i2c_reg_read_byte_dt(&accelerometer, 0x22, &gyro_x1);
		i2c_reg_read_byte_dt(&accelerometer, 0x23, &gyro_x2);
		// Read gyroscope data for Y axis
		i2c_reg_read_byte_dt(&accelerometer, 0x24, &gyro_y1);
		i2c_reg_read_byte_dt(&accelerometer, 0x25, &gyro_y2);
		// Read gyroscope data for Z axis
		i2c_reg_read_byte_dt(&accelerometer, 0x22, &gyro_z1);
		i2c_reg_read_byte_dt(&accelerometer, 0x23, &gyro_z2);
		// Read linear acceleraion for X axis
		i2c_reg_read_byte_dt(&accelerometer, 0x28, &acc_x1);
		i2c_reg_read_byte_dt(&accelerometer, 0x29, &acc_x2);
		// Read linear acceleraion for Y axis
		i2c_reg_read_byte_dt(&accelerometer, 0x2a, &acc_y1);
		i2c_reg_read_byte_dt(&accelerometer, 0x2b, &acc_y2);
		// Read linear acceleraion for Z axis
		i2c_reg_read_byte_dt(&accelerometer, 0x2c, &acc_z1);
		i2c_reg_read_byte_dt(&accelerometer, 0x2d, &acc_z2);

		k_mutex_unlock(&gyro_acc_mutex); // unlock mutex after reading and storing datas
	}
	else
	{}
}

void data_on_accelerometer(const struct device *dev, struct gpio_callback *cb,
						   uint32_t pins)
{
	k_work_submit(&read_display_accelerometer_work);
}

void main(void)
{
	// Variable for interrupt line
	int ret;
	// Init work object and associate with read and display data function
	k_work_init(&read_display_accelerometer_work, read_display_accelerometer);

	// Check if accelerometer is ready
	if (!device_is_ready(accelerometer.bus))
	{
		printk("Error: i2c bus device is not ready");
		return;
	}

	// Set interruption line as input
	ret = gpio_pin_configure_dt(&accelerometer_irq, GPIO_INPUT);
	if (ret != 0)
	{
		printk("Error %d: failed to configure %s pin %d\n",
			   ret, accelerometer_irq.port->name, accelerometer_irq.pin);
		return;
	}

	// Set interruption line to trigger interruption when edge rising
	ret = gpio_pin_interrupt_configure_dt(&accelerometer_irq, GPIO_INT_EDGE_RISING);
	if (ret != 0)
	{
		printk("Error %d: failed to configure interrupt on %s pin %d\n",
			   ret, accelerometer_irq.port->name, accelerometer_irq.pin);
		return;
	}

	// Set callback function on accelerometer
	gpio_init_callback(&accelerometer_data, data_on_accelerometer, BIT(accelerometer_irq.pin));
	gpio_add_callback(accelerometer_irq.port, &accelerometer_data);

	// Set accelerometer characteristics
	i2c_reg_write_byte_dt(&accelerometer, 0x12, 0b00000101); // reset accelerometer and keep automatic incrementation of adresses enabled (CTRL3_C)
	i2c_reg_write_byte_dt(&accelerometer, 0x15, 0b00010000); // set XL_HM_MODE to 1 to disable high performance mode
	i2c_reg_write_byte_dt(&accelerometer, 0x10, 0b10110000); // set ODR_XLR[3:0] to 1 0 1 1 for a frequency at 1.6 Hz and FS_XL[1:0] to 0 0 to have measurements beetween -2g(0) and 2g(65535)
	// Set gyroscope characteristics
	i2c_reg_write_byte_dt(&accelerometer, 0x11, 0b01000000); // set measurements frequency for gyroscope at 104 hz (CTRL2_G register)
	// Set interruption characteristics
	i2c_reg_write_byte_dt(&accelerometer, 0x0d, 0b00000001); // set INT1_DRDY_XL (data on acceleromter) to 1 (enable) on the interruption register INT1_CTRL
}

// Define thread for complementary filter
K_THREAD_DEFINE(filter_id, STACKSIZE, complementary_filter, NULL, NULL, NULL, PRIORITY, 0, 0);
