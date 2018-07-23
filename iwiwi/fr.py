import glob
import hashlib
import subprocess
import time

def md5(fname):
    hash_md5 = hashlib.md5()
    with open(fname, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hash_md5.update(chunk)
    return hash_md5.hexdigest()


def create_md5_map(paths):
    return {
        md5(path): path.split('/')[-1].split('_')[0]
        for path in paths
    }

def main():
    fa_paths = glob.glob('../data/problemsF/FA*_tgt.mdl')
    fa_map = create_md5_map(fa_paths)

    # print(fa_map)

    fd_paths = glob.glob('../data/problemsF/FD*_src.mdl')
    fd_map = create_md5_map(fd_paths)

    print(fd_map)

    fr_src_paths = glob.glob('../data/problemsF/FR*_src.mdl')
    for fr_src_path in fr_src_paths:
        fr_name = fr_src_path.split('/')[-1].split('_')[0]
        fr_tgt_path = fr_src_path.replace('_src', '_tgt')
        print(fr_src_path, fr_tgt_path)

        fr_src_md5 = md5(fr_src_path)
        fr_tgt_md5 = md5(fr_tgt_path)

        problem_num = int(fr_name[2:])
        if problem_num <= 81:
            continue

        if (fr_src_md5 in fd_map) and (fr_tgt_md5 in fa_map):
            fd_name = fd_map[fr_src_md5]
            fa_name = fa_map[fr_tgt_md5]

            fd_path = 'unagi-20180724-000641/{}.nbt'.format(fd_name)
            fa_path = 'unagi-20180724-000641/{}.nbt'.format(fa_name)

            print(' {}'.format(fd_path))
            print(' {}'.format(fa_path))
            sol1 = open(fd_path, 'rb').read()
            sol2 = open(fa_path, 'rb').read()

            sol = sol1[:-1] + sol2
            print(len(sol1), len(sol2), len(sol))

            sol_path = 'out/{}.nbt'.format(fr_name)
            with open(sol_path, 'wb') as f:
                f.write(sol)

            try:
                subprocess.check_call('unagi-submit --nbt_file={} --problem {}'.format(sol_path, fr_name), shell=True)
            except:
                pass

            time.sleep(1)

            # break


if __name__ == '__main__':
    main()
