#include <iostream>

#include "kyber.hpp"
#include "hash.hpp"
#include "base.hpp"

int main() {
	unsigned char* inArr = (unsigned char*)"Sha3";
	unsigned char* out = sha3::sha3(inArr, strlen((char*)inArr), 256);
	std::cout << base16::encode(out, 256/8) << std::endl;

	return 0;
}
