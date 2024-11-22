#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <liburing.h>

// 宿主环境提供的接口函数
int wasi_fd_write(int fd, const struct iovec *iov, int iovcnt, size_t *nbytes) {
    struct io_uring ring;
    struct io_uring_sqe *sqe;
    struct io_uring_cqe *cqe;
    int ret, res;
    char *buf = iov[0].iov_base;
    size_t len = iov[0].iov_len;

    // 创建 io_uring 实例
    if (io_uring_queue_init(8, &ring, 0) < 0) {
        perror("io_uring_queue_init");
        return -1;
    }

    // 获取 SQE
    sqe = io_uring_get_sqe(&ring);
    if (!sqe) {
        perror("io_uring_get_sqe");
        return -1;
    }

    // 填充 SQE 信息
    io_uring_prep_write(sqe, fd, buf, len, -1);

    // 提交请求
    ret = io_uring_submit(&ring);
    if (ret < 0) {
        perror("io_uring_submit");
        return -1;
    }

    // 等待完成
    do {
        ret = io_uring_wait_cqe(&ring, &cqe);
        if (ret < 0) {
            perror("io_uring_wait_cqe");
            return -1;
        }
        res = cqe->res;
        io_uring_cq_advance(&ring, 1);
    } while (res < 0);

    // 清理
    io_uring_queue_exit(&ring);

    *nbytes = res;
    return 0;
}

int main() {
    // 示例调用
    char buffer[] = "Hello, io_uring!";
    struct iovec iov = { .iov_base = buffer, .iov_len = sizeof(buffer) };
    size_t nbytes;
    wasi_fd_write(1, &iov, 1, &nbytes);
    printf("Wrote %zu bytes\n", nbytes);
    return 0;
}