#include <stdio.h>
#include <stdlib.h>
#include "cblas.h"

int main(){
	size_t N = 1024 * 1024 * 512;
	double *x = (double *)malloc(sizeof(double) * N);
	double *y = (double *)malloc(sizeof(double) * N);
	double a = 2.0;
	for (int i=0;i<N;i++){
		x[i] = i;
		y[i] = i;
	}
	cblas_daxpy(N,a,x,1,y,1);


	//	for (int i=0;i<N;i++){
	//			y[i] += a * x[i];
	//		}
	//	for (int i=0;i<2;i++){
	//			printf("%lf,",y[i]);
	//		}

		
	free(x);
	free(y);

}
