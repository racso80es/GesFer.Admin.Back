import re

def regex_replace(filepath, pattern, replace):
    with open(filepath, 'r') as f:
        content = f.read()
    content = re.sub(pattern, replace, content)
    with open(filepath, 'w') as f:
        f.write(content)

# AdminJsonDataSeeder implements IAdminJsonDataSeeder but where is it?
