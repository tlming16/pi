import setuptools
import subprocess as sp
import os
with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()


def get_compiler_info():
    res = sp.check_output(["gcc", "-v"], stderr=sp.STDOUT)
    res = res.decode('utf-8')
    items = res.split('\n')
    for res in items:
        if "version" in res:
            item = res.split(' ')
            for ind in range(len(item)):
                if item[ind] == "version":
                    compiler = item[ind - 1]
                    version = item[ind + 1]
                    return compiler, version
    return None,None


def build(cpp_file,dest_dir,threads=True):
    cmd="g++ -fPIC -shared -O3 -o {} -std=c++11 {} "
    if threads:
        cmd += "-DPI_THREAD"
    cmd=cmd.format( dest_dir,cpp_file)
    try:
        os.system(cmd)
    except:
        exit(1)

gcc_info =get_compiler_info()
if gcc_info[0] is None:
    exit(1)
gcc_compiler= gcc_info[0]
gcc_version = gcc_info[1]
version = int(gcc_version.split('.')[0])
flag=True
if gcc_compiler=="gcc":
    if version<5 :
        flag=False
else:
    if version<11:
        flag=False

build("src/pi_compute/include/pi.cpp"," src/pi_compute/lib/lib_pi.so",flag)

setuptools.setup(
    name="pi_compute",
    version="3.14.1592",
    author="mathm",
    author_email="tlming16@fudan.edu.cn",
    description="to compute N digits of pi",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/tlming16/pi",
    project_urls={
        "Bug Tracker": "https://github.com/tlming16/pi-/issues",
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    package_dir={"": "src"},
    package_data={"pi_compute":["include","include/pi.cpp","lib/lib_pi.so"],"test":["*.cpp"],},
    packages=['pi_compute'],
    python_requires=">=3.6",
)
