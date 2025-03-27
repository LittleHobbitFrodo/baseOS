//	integers.h
//	this file originally belonged to baseOS project
//		an OS template on which to build

//  defining int types -> like <stdint.h>
	///	NOTE: floating points may be problematic in OS kernel

#pragma once

//  8bit
typedef signed char i8;
typedef unsigned char u8;

//  16bit
typedef signed short int i16;
typedef unsigned short int u16;

//  32bit
typedef signed int i32;
typedef unsigned int u32;

//  64bit
typedef signed long int i64;
typedef unsigned long int u64;

//  128 bit
typedef signed long long int i128;
typedef unsigned long long int u128;

typedef u8 byte;
typedef u16 word;
typedef u32 dword;
typedef u64 qword;

#define true 1
#define false 0
#define NULL (void*)0

typedef u8 bool;


//	MAX and MIN
#define CHAR_MIN (-128)
#define CHAR_MAX 127

#define I8_MIN (-128)
#define I8_MAX 127
#define U8_MIN 0
#define U8_MAX 255

#define I16_MIN (-32768)
#define I16_MAX 32767
#define U16_MIN 0
#define U16_MAX 65535

#define I32_MIN (-2147483648)
#define I32_MAX 2147483647

#define U32_MIN 0
#define U32_MAX 4294967295

#define I64_MIN (-9223372036854775808)
#define I64_MAX 9223372036854775807

#define U64_MIN 0
#define U64_MAX 18446744073709551615



//  size_t
#ifndef _SIZE_T_DEFINED
	#define _SIZE_T_DEFINED
	#ifdef __i386__
		typedef u32 size_t;
		typedef i32 ssize_t;
	#elif defined(__x86_64__) || defined(__arm__) || defined(__aarch64__)
		typedef u64 size_t;
		typedef i64 ssize_t;
	#elif defined(__powerpc__)
		typedef u32 size_t;
		typedef i32 ssize_t;
	#elif defined(__powerpc64__)
		typedef u64 size_t;
		typedef i64 ssize_t;
	#else
		#error could not determine size_t, please define it before (_SIZE_T_DEFINED)
	#endif
#endif
