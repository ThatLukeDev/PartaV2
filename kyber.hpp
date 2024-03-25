#ifndef tld_kyber
#define tld_kyber

namespace kyber {
	unsigned char* BytesToBits(unsigned char* B, unsigned int k) {
		unsigned char _B = malloc(k*8);

		for (unsigned int i = 0; i < k; i++) {
			_B[i*8+0] = (B & 0b00000001) ? 1 : 0;
			_B[i*8+1] = (B & 0b00000010) ? 1 : 0;
			_B[i*8+2] = (B & 0b00000100) ? 1 : 0;
			_B[i*8+3] = (B & 0b00001000) ? 1 : 0;
			_B[i*8+4] = (B & 0b00010000) ? 1 : 0;
			_B[i*8+5] = (B & 0b00100000) ? 1 : 0;
			_B[i*8+6] = (B & 0b01000000) ? 1 : 0;
			_B[i*8+7] = (B & 0b10000000) ? 1 : 0;
		}

		return _B;
	}
}

#endif
