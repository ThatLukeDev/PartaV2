#ifndef tld_base
#define tld_base

namespace base16 {
	char numerals[] = {
		'0', '1', '2', '3', '4', '5', '6', '7',
		'8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
	};

	const char* encode(void* data, unsigned int size) {
		char* _out = (char*)malloc(size * 2);

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

#endif
