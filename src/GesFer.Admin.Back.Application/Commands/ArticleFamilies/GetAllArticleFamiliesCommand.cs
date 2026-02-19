using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.ArticleFamilies;

namespace GesFer.Admin.Back.Application.Commands.ArticleFamilies;

public record GetAllArticleFamiliesCommand(Guid? CompanyId = null) : ICommand<List<ArticleFamilyDto>>;
