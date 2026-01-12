import shutil
import sys
from urllib.parse import urlparse, urlunparse
from pathlib import Path

TEMPLATE_FOLDER = Path("./template")

def main():
    # Check if at least one argument (besides the script name) was provided
    if len(sys.argv) != 2:
        print("Please provide the kattis nus url")
        return

    original_nus_url = urlparse(sys.argv[1])
    problem_path = original_nus_url.path.split("/")
    problem_id = problem_path[-1]
    for i in range(len(problem_path)):
        if problem_path[i] == "problems":
            actual_problem_path = problem_path[i:]
            break
    open_kattis_url = original_nus_url._replace(netloc=original_nus_url.netloc.replace("nus", "open"), path="/"+"/".join(actual_problem_path))
    print(urlunparse(open_kattis_url))

    problem_path = Path(f"./{problem_id}")
    if problem_path.exists():
        print(f"problem folder already exists at ./{problem_id}")
        return
    else:
        try:
            shutil.copytree(TEMPLATE_FOLDER, problem_path)
            print(f"problem folder created, {problem_path.as_posix()}")
        except Exception as e:
            print(f"Failed to create problem folder, err: {e}")


if __name__ == "__main__":
    main()
