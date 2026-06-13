import re

def regex_replace(filepath, pattern, replace):
    with open(filepath, 'r') as f:
        content = f.read()
    content = re.sub(pattern, replace, content)
    with open(filepath, 'w') as f:
        f.write(content)

filepath = 'src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs'
regex_replace(filepath, r'public async Task<AdminSeedResult> SeedAllAsync\(\)', r'public async Task<AdminSeedResult> SeedAllAsync(CancellationToken cancellationToken = default)')

methods = [
    'SeedLanguagesAsync', 'SeedCountriesAsync', 'SeedStatesAsync', 'SeedCitiesAsync',
    'SeedPostalCodesAsync', 'SeedCompaniesAsync', 'SeedUsersAsync', 'SeedAdminUsersAsync'
]

for method in methods:
    regex_replace(filepath, f'await {method}\\(\\)', f'await {method}(cancellationToken)')
