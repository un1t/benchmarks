from django.db import models


class Word(models.Model):
    title = models.CharField(max_length=128)
    content = models.TextField()

    class Meta:
        db_table = "words"
