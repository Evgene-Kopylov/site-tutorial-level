import os
from pprint import pprint

md_filename = "project_code.md"


def get_project_files(directory="./"):
    file_list = []
    ignore_dirs = [
        'venv',
        'node_modules',
        'target',
        '.git',
        '.idea',
    ]

    for root, dirs, files in os.walk(directory):
        for ignd in ignore_dirs:
            if ignd in dirs:
                dirs.remove(ignd)
        for file in files:
            # if file.endswith(".py"):  # Фильтруем только .py файлы
            if file.startswith("LICENSE") \
                    or file == md_filename \
                    or file.endswith(".lock"):
                continue
            try:
                file_list.append(os.path.join(root, file))
            except:
                continue
    return file_list


# Получите список файлов в проекте
project_directory = "./"
file_list = get_project_files(project_directory)

# Создайте новый Markdown-файл

# Откройте файл для записи
with open(md_filename, "w", encoding="utf-8") as md_file:
    # Добавьте содержимое каждого файла Python в Markdown-файл
    for file in file_list:
        with open(file, "r", encoding="utf-8") as python_file:
            try:
                code = python_file.read()
                md_file.write(f"```python\n# {file}\n{code}\n```\n")
            except:
                print("SKIP", file)

print(f"Собраны файлы в {md_filename}")

if __name__ == "__main__":
    pprint(get_project_files())
