#ifndef matrices
#define matrices

#include <limits>
#include <cstring>

template <typename T, int columns, int rows>
class matrix {
public:
	T arr[columns][rows];

	matrix() {
		for (int x = 0; x < columns; x++) {
			for (int y = 0; y < rows; y++) {
				arr[x][y] = 0;
			}
		}
	}

	T* operator [](int x) {
		return arr[x];
	}

	matrix<T, rows, columns> transpose() {
		matrix<T, rows, columns> tpout = matrix<T, rows, columns>();

		for (int x = 0; x < columns; x++) {
			for (int y = 0; y < rows; y++) {
				tpout.arr[y][x] = 0;
			}
		}

		return tpout;
	}

	matrix<T, columns, rows> operator=(matrix<T, columns, rows> p) {
		for (int x = 0; x < columns; x++) {
			for (int y = 0; y < rows; y++) {
				this->arr[x][y] = p.arr[x][y];
			}
		}

		return *this;
	}

	matrix<T, columns, rows> operator+(T val) {
		matrix<T, columns, rows> out = matrix<T, columns, rows>();

		for (int y = 0; y < rows; y++) {
			for (int x = 0; x < columns; x++) {
				out[x][y] = (*this)[x][y] + val;
			}
		}

		return out;
	}

	matrix<T, columns, rows> operator-(T val) {
		matrix<T, columns, rows> out = matrix<T, columns, rows>();

		for (int y = 0; y < rows; y++) {
			for (int x = 0; x < columns; x++) {
				out[x][y] = (*this)[x][y] - val;
			}
		}

		return out;
	}

	matrix<T, columns, rows> operator*(T val) {
		matrix<T, columns, rows> out = matrix<T, columns, rows>();

		for (int y = 0; y < rows; y++) {
			for (int x = 0; x < columns; x++) {
				out[x][y] = (*this)[x][y] * val;
			}
		}

		return out;
	}

	matrix<T, columns, rows> operator+(matrix<T, columns, rows> p) {
		matrix<T, columns, rows> out = matrix<T, columns, rows>();

		for (int y = 0; y < rows; y++) {
			for (int x = 0; x < columns; x++) {
				out[x][y] = (*this)[x][y] + p[x][y];
			}
		}

		return out;
	}

	matrix<T, columns, rows> operator-(matrix<T, columns, rows> p) {
		matrix<T, columns, rows> out = matrix<T, columns, rows>();

		for (int y = 0; y < rows; y++) {
			for (int x = 0; x < columns; x++) {
				out[x][y] = (*this)[x][y] - p[x][y];
			}
		}

		return out;
	}

	template <int pcols, int prows>
	matrix<T, pcols, rows> operator*(matrix<T, pcols, prows> p) {
		matrix<T, pcols, rows> out = matrix<T, pcols, rows>();

		for (int x = 0; x < pcols; x++) {
			for (int y = 0; y < rows; y++) {
				for (int i = 0; i < prows; i++) {
					out[x][y] += (*this)[i][x] * p[y][i];
				}
			}
		}

		return out;
	}
};

template <typename T, int columns, int rows>
std::ostream& operator<<(std::ostream& os, matrix<T, columns, rows> m) {
	for (int y = 0; y < rows; y++) {
		for (int x = 0; x < columns; x++) {
			os << m[x][y] << "\t";
		}
		if (y < rows - 1) {
			os << "\n";
		}
	}
	return os;
}

#endif
