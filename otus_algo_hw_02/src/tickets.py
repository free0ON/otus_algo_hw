import unittest
import numpy

class TestTickets(unittest.TestCase):

    # def test_brut_count(self):
    #     path = "1.Tickets/test."
    #     for i in range(4):
    #         input_file = open(path + str(i) + ".in", "r")
    #         output_file = open(path + str(i) + ".out", "r")  
    #         input = int(input_file.readline())
    #         output = numpy.uint64(output_file.readline())
    #         self.assertEqual(output,brut_get_happy_count(input))
    #         print("Brut count: ", input, output)
    #         input_file.close()
    #         output_file.close()

    def test_fast_count(self):
        path = "1.Tickets/test."
        for i in range(10):
            input_file = open(path + str(i) + ".in", "r")
            output_file = open(path + str(i) + ".out", "r")  
            input = int(input_file.readline())
            output = numpy.uint64(output_file.readline())
            self.assertEqual(output,fast_get_happy_count(input))
            print("python fast count: ", input, " : ", output)
            input_file.close()
            output_file.close()


def get_sum_number (number, n):
    sum = numpy.uint64(0)
    while number > 0:
        sum += number % 10
        number //= 10
    return sum #(map(int, list(str(_number))))

def brut_get_happy_count(n):
    count = 0
    max_num = pow(10,n)
    for left in range(0, max_num):
        for right in range(0, max_num):
            if get_sum_number(left, n) == get_sum_number(right, n):
                count += 1
    return count

def fast_get_happy_count(n):
    count = numpy.uint64(0)
    max_num = numpy.uint64(pow(10,n))
    sums_number = numpy.uint64(9*n + 1)
    # sums = numpy.array([0]*sums_number,dtype='uint64')
    sums = [0]*sums_number
    for number in range(max_num):
        number = numpy.uint64(number)
        index = get_sum_number(number, n)
        sums[index] += 1; 

    for i in range(sums_number):
        count += sums[i] * sums[i]
    
    # print("Sum num: \n", sums_number)
    # print(sums)

    return count

def test_suite():
    suite = unittest.TestSuite()
    suite.addTest(TestTickets('test_fast_count'))
    return test_suite

if __name__ == '__main__':
    unittest.TestProgram()
