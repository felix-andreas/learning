# notes: conversational ai with rasa

## open questions (discuss with valerie)

* how does ted build up a dialog policy from stories?
* 5.1 Can a single word in a sentence be part of two entities?
* what is `UnexpecTEDIntentPolicy2`?
* look at contents of model.tar.gz

## ideas

* refactor `rock_paper_scissor_play` -> `play_game` + `entity`

## lesson 1: introduction to rasa

* rasa is framework to build task oriented dialog systems
* two systems:
    * NLU
        * rule-based (regex)
        * neural (DIET)
    * dialog policies
        * rule-based
        * neural (TED)
* conversation-driven-development: manually review and correcterrors in conversations

### questions

* nlu is for transforming text into information. the dialog policy decides what to do next.
* rule-based is more light-weight and requires domain knowledge. neural methods are better at generalizing but require a lot of training data.

## lesson 2: creating a new assistant

* rasa cli
    * `rasa --help` - show help
    * `rasa init` - creates a scaffold for a new project
    * `rasa train` - trains new model and stores it in the `model` directory (name is derived from time it was trained)
    * `rasa shell` - talk to assistant in the shell
        * type `/stop` to leave `rasa shell`
        * type `/restart` to restart the conversation `rasa shell`
        * set the `--debug` flag for addtional information
* important files
    * `domain.yml` - defines all available intents & actions
    * `config.yml` - defines NLU pipeline. (has sensible default)
    * data files:
        * `nlu.yml` - examples of how to express an intent
        * `stories.yml` - shows potential conversation flows
        * `rules.yml` - defines conditional rules

### questions

* `pip install rasa` (or poetry, etc.)
* `domain.yml` is config for domain knowledge. `config.yml` is config for ml pipeline
* `rasa train` and `rasa shell`

## lesson 3: the domain file

* the `domain.yml` file defines everything the assistant is aware about
    * **responses** - what assistant says to user
    * **intents** - categories of things a user says
    * **slots** - variables that a stored over the course of the conversation
    * **entities** - information extracted from the text
    * **forms & actions** - application logic and what assistant can do
* **responses** - things asssistant says to user
    * starts with `utter_` and has `text` field
    * if multiple texts are given, one will be selected randomly
    * use slots in your responses (is `None` until it's filled)
    * defines `buttons`, has (`text` and `payload` fields)
    * include images (rendering depends on channel)
    * define `channel` specific repsonses (e.g. slack)
* **intents** - things user says to assistant
    * tip - start with small number of intents
    * training data for the intents is defined in the `nlu.yml` file

## lesson 4: training data & rules

* data
    * data of pretained model
    * user-generated text
    * patterns of conversations
    * where to get data (examples):
        * customer support logs
        * user conversations with your assistant
* how should conversations go (dialog policy)
    * stories - training data to teach assistant what to do next
    * rules
* how do user say things
    * intents
* `stories.yml`
    * use `or` to accept multiple intents within a story path
    * applies machine learning to develop a dialog policy
    * use `rasa interactive` to build stories
* `rules.yml` 
    * these are fixed rules which don't require any machine learning
    * you can create small stories (they don't have to be top to bottom)
    * use real user conversations (see rasa x)
* `nlu.yml`
    * training data for intents
    * use small number of intents
    * use user data for intent examples
    * don't use to much intents
        * more training data -> more time to train
        * harder to maintain
        * harder to annotate
        * transformers scale linearly with number of classes -> worse performance
        * example: instead of multiple buy intents use one buy intent and extract an entity

### questions

* if multiple intents are appropriate
* use checkpoints to compose stories
* no. start with small number of intents. better performance of transformer model and easier to maintain

## lesson 5: entities

* piece of information within user's text
    * number, date, country name, product name, destination
* label entities in intents in `nlu.yml`
* three ways to extract entities
    1. pre built models (`config.yml`)
        * duckling 
        * spacy
    2. regex
        * define in `nlu.yml`
    3. machine learning
        * extract custom entities
        * rasa comes with some models (DIET)
* `synonym`: map multiple names to single name
    * can be defined in `nlu.yml`
    * define inline with intents
* `lookup` - lookup tables
    * list of words to generate case-sensitive regular expression patterns
* entity role and groups
    * example: entitiy location, role: origin/destination
     
### questions

* ??
* pre-built models, regex, machine learning

## lesson 6: slots

* slots are assistant's memory
* can be used later
* defined in `domain.yml`
* there are two main ways to set slots
    1. NLU
    2. custom actions
* slots can influence flow of conversation
    * `slot_was set` in `stories.yml`
* slot mappings
    * defines how slots are filled
    * are applied after each user message
    * use `intent` and `not_intent` include/excludecertain intents from slot mapping
    * types of slot mappings
        * `from_entity` fill slot based on extracted entities
        * `from_text` - fill with latest user message
        * `from_intent` - fill with predefined value if intent is predicted
        * `from_trigger_intent` - fill slot if form is activated by user
        * `custom` - write custom slot validation action
* slot types
    * `text` - only presence influences conversation
    * `boolean` -  store boolean value (e.g. `isAuthenticated`)
    * `categorical` - e.g. low, medium high
    * `float` - store float, optional min, max values
    * `list` - store list of values, e.g. grocery items: cookies, milk, chocolate. only presence has influence on conversation
    * `any` - has no influence on conversation
* `initial_value` can be used to set default value

### questions

* depends on slot type and if `influence_conversation` field is set
* slot is a place to store information. an entity is extracted from text. often entities are stored in slots

## lesson 7: responses

* respones are simple messages that the assistant sends to the user
* use responses in stories to enable them in assistant
* there are defined in the `responses` section of the `domain.yml` file 
* prefix `utter_` is important to distinguish between different response types
* create multiple responses for a specific template
* variables: include slots in responses to make themore dynamic
* start rasax with `rasa x`
* content responses
    * include images -> add `image` field to a response definition
    * add buttons
    * define custom payload
        * depends if channels supports type
    * `markdown`
    * specific response based on channel (e.g. slack)

### questions

* text, button, images, etc. depends on channel
* use `channel` field to define channel-specific respones


