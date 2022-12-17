from django.http import HttpResponse, JsonResponse
from .models import Word


def index(request):
    words = list(Word.objects.values()[:100])
    return JsonResponse(words, safe=False)


def ping(request):
    return HttpResponse("OK")
