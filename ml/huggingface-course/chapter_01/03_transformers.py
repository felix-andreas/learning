#%%
from transformers import pipeline

#%% Sentiment Analysis
classifier = pipeline("sentiment-analysis")
result = classifier(
    [
        "I've been waiting for a HuggingFace course my whole life.",
        "I don't know how NLP works yet, but I will probably find out thanks to the Huggingface NLP course",
    ]
)
print(result)

#%% Zero-shot classification
classifier = pipeline("zero-shot-classification")
result = classifier(
    "Why is Rust not used for machine learning more often?",
    candidate_labels=["Programming", "Food", "Music"],
)
print(result)

#%% Text Generation
generator = pipeline("text-generation")
result = generator(
    "By default the pipeline for text-generation of the huggingface's transformers library uses the a model called "
)
print(result)

# %%
