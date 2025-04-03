import secrets
import os

MOD = 1 << 64

def random_u64():
    return secrets.randbits(64)

def generate_triples(l, num_triples):
    # 创建输出目录：如 ./2/
    output_dir = os.path.join(".", str(l))
    os.makedirs(output_dir, exist_ok=True)
    
    # 打开 l 个文件用于写入
    files = [open(os.path.join(output_dir, f"triples_P_{i}.txt"), "w") for i in range(l)]
    
    for _ in range(num_triples):
        # 生成随机向量 a, b
        a = [random_u64() for _ in range(l)]
        b = [random_u64() for _ in range(l)]
        
        # 计算目标 c 的和
        sum_a = sum(a) % MOD
        sum_b = sum(b) % MOD
        target_c_sum = (sum_a * sum_b) % MOD
        
        # 随机生成前 l-1 个 c，最后一个用来修正
        c = [random_u64() for _ in range(l - 1)]
        current_c_sum = sum(c) % MOD
        c.append((target_c_sum - current_c_sum) % MOD)
        
        # 写入每个 triples_P_i.txt
        for i in range(l):
            files[i].write(f"a: {a[i]}, b: {b[i]}, c: {c[i]}\n")
    
    # 关闭文件
    for f in files:
        f.close()

# 示例调用
if __name__ == "__main__":
    num_triples = 1000000
    num = [3,11,21]
    for i in num:
        generate_triples(i, num_triples)
        print(f"成功生成 {i} 个文件，保存在当前目录的 ./{i}/ 下。")