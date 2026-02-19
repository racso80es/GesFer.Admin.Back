using GesFer.Admin.Back.Application.Common.Interfaces;

namespace GesFer.Admin.Back.Application.Commands.ArticleFamilies;

public record DeleteArticleFamilyCommand(Guid Id) : ICommand;
