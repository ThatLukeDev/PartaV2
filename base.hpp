#ifndef tld_base
#define tld_base

#include <cstring>
#include <cmath>

namespace base16 {
	char numerals[] = {
		'0', '1', '2', '3', '4', '5', '6', '7',
		'8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
	};

	const char* encode(void* data, unsigned int size) {
		char* _out = (char*)malloc(size * 2 + 1);
		_out[size*2] = 0;

		for (unsigned int i = 0; i < size; i++) {
			_out[i*2] = numerals[((unsigned char*)data)[i] / 16];
			_out[i*2+1] = numerals[((unsigned char*)data)[i] % 16];
		}

		return (const char*)_out;
	}

	void* decode(const char* str) {
		unsigned int size = strlen(str);

		char* _out = (char*)malloc(size / 2);

		for (unsigned int i = 0; i < size / 2; i++) {
			unsigned char val = 0;

			for (int j = 0; j < sizeof(numerals); j++) {
				if (str[i*2] == numerals[j]) {
					val += j * 16;
				}
				if (str[i*2+1] == numerals[j]) {
					val += j;
				}
			}

			_out[i] = val;
		}

		return _out;
	}
}

namespace base64 {
	char numerals[] = {
		'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
		'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
		'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
		'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
		'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
		'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
		'w', 'x', 'y', 'z', '0', '1', '2', '3',
		'4', '5', '6', '7', '8', '9', '+', '/'
	};
	char pad = '=';

	const char* encode(void* data, unsigned int size) {
		if (size < 1) {
			return nullptr;
		}

		unsigned int blocks = std::ceil(size / 3.0);
		unsigned int leaves = (size % 3);
		if (leaves != 0) leaves = 3 - leaves;

		char* _data = (char*)malloc(blocks * 3);
		for (int i = 0; i < blocks * 3; i++) _data[i] = 0;
		memcpy(_data, data, size);

		char* _out = (char*)malloc(blocks * 4);

		for (unsigned int i = 0; i < blocks; i++) {
			_out[i*4] = numerals[_data[i*3] >> 2];
			_out[i*4+1] = numerals[((_data[i*3] << 4) & 0b00110000) | (_data[i*3+1] >> 4)];
			_out[i*4+2] = numerals[((_data[i*3+1] << 2) & 0b00111100) | (_data[i*3+2] >> 6)];
			_out[i*4+3] = numerals[_data[i*3+2] & 0b00111111];
		}

		if (leaves > 0) {
			_out[blocks*4-1] = pad;
		}
		if (leaves > 1) {
			_out[blocks*4-2] = pad;
		}

		free(_data);

		return (const char*)_out;
	}

	void* decode(const char* str) {
		int len = strlen(str);

		if (len < 1) {
			return nullptr;
		}

		unsigned int blocks = std::ceil(len / 4.0);

		char* _out = (char*)malloc(blocks * 3);
		for (int i = 0; i < blocks * 3; i++) _out[i] = 0;

		for (unsigned int i = 0; i < blocks; i++) {
			for (unsigned int j = 0; j < sizeof(numerals); j++) {
				if (str[i*4] == numerals[j]) {
					_out[i*3] |= j << 2;
				}
				if (str[i*4+1] == numerals[j]) {
					_out[i*3] |= j >> 4;
					_out[i*3+1] |= j << 4;
				}
				if (str[i*4+2] == numerals[j]) {
					_out[i*3+1] |= j >> 2;
					_out[i*3+2] |= j << 6;
				}
				if (str[i*4+3] == numerals[j]) {
					_out[i*3+2] |= j;
				}
			}
		}

		return _out;
	}
}

#endif
