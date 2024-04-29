#include <iostream>

#include "kyber.hpp"
#include "hash.hpp"
#include "base.hpp"

int main() {
	unsigned char* inArr = (unsigned char*)"Keccak-256";
	unsigned char* out = sha3::keccak(inArr, strlen((char*)inArr), 256, 512);
	std::cout << base16::encode(out, 256/8) << std::endl;

	return 0;
}
