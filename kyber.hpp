#ifndef tld_kyber
#define tld_kyber

namespace kyber {
	const int n = 256;
	const int n_ = 9;
	const int q = 3329;

	unsigned char* BytesToBits(unsigned char* B, unsigned int k) {
		unsigned char* _B = (unsigned char*)malloc(k*8);

		for (unsigned int i = 0; i < k; i++) {
			_B[i*8+0] = (B[i] & 0b00000001) ? 1 : 0;
			_B[i*8+1] = (B[i] & 0b00000010) ? 1 : 0;
			_B[i*8+2] = (B[i] & 0b00000100) ? 1 : 0;
			_B[i*8+3] = (B[i] & 0b00001000) ? 1 : 0;
			_B[i*8+4] = (B[i] & 0b00010000) ? 1 : 0;
			_B[i*8+5] = (B[i] & 0b00100000) ? 1 : 0;
			_B[i*8+6] = (B[i] & 0b01000000) ? 1 : 0;
			_B[i*8+7] = (B[i] & 0b10000000) ? 1 : 0;
		}

		return _B;
	}
}

#endif
