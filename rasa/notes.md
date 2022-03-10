# notes: conversational ai with rasa

## open questions (discuss with valerie)

* how does ted build up a dialog policy from stories?
* 5.1 Can a single word in a sentence be part of two entities?
* what is `UnexpecTEDIntentPolicy2`?
* look at contents of model.tar.gz
* what is featurization, is this the encoder part of the transformers model?? (lesson 8)
* what are classifiers. are they built on top of the transfomers model? (model heads?) (lesson 8)
* why is our pipeline emtpy? what is the default? (lesson 8)
* why can nlu pipeline only be trained and used sequentially but dialog policy pipeline can be trained and used in parallel (lesson 8)
* difference between sparse vs dense features (lesson 8)
* always `return []` after `dispatcher.utter_message(...)`


## ideas

* refactor `rock_paper_scissor_play` -> `play_game` + `entity`
* use `tracker.get_latest_entity_value` instead of `get_slot`
* lesson 9: he added a `RegexEntityExtractor` to the pipeline to detect the `place` entity. maybe this is the reason our tests fail
* implement `utter_info` - display info of what emilio is able to do
    * tell time
    * play 2 games: schere stein papier, guess number
    * use some API: e.g. geographic - tell distance between two different location
    * action which sets slot using `SlotSet`

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

## lesson 8: pipeline and policy configuration

* training pipelines and dialoage policy are the core of the assistant
    * defines how the assistant understands user input  
    * defines how to respond back to the user
* `config.yml`
    * `pipeline`
        * defines the NLU training pipeline, which defines the steps that will be used to extract intents and entities
        * from raw message tokenization to which classifier will be used 
            * tokenization
            * featurization
            * intent classification & entity extraction
        * order of components matters
    * `policies`
        * defines dialog policies and models
* overview of components
    * **tokenizer** - parse user input into tokens
    * **featurizer** - extract features from tokens
    * **classifiers** - use features to put labels on user's input (DIET - Dual Intent and Entity Transformer)
    * **entity extractors** - you can multiple entity extractors (DIET, Regex, and Duckling)
* dialog policies
    * what is the best next action?
    * either rule-based (`rules.yml`) or ml-based (`stories.yml`)
    * in contrast to nlu pipeline (trained and used sequentielly), dialog policies can be trained and used in parallel
    * priorities can be used to define the precedence between different policies
    * default policy priority in rasa
        * 6 - `RulePolicy
        * 3 - `MemoizationPolicy`
        * 1 - `TEDPolicy`
    * `RulePolicy` - main very strict rule-based policy in rasa
    * `MemoizationPolicy` - remembers stories from `stories.yml` and matches it with current conversation. has confidence of 1
    * `TEDPolicy` - Transformer Embedding Dialoge Policy
        * `max_history` - defines how many steps of the conversation the assistant keeps in memory for making the prediction
    * custom components can also be build in python

### questions

* tokenizer, featurizer, classifier (entity or intent extraction)
* because policies might have the same confidence

## lesson 9: custom actions

* allows to write custom code in python which can run of the behave of the users
    * do a computation
    * fetch data from database
    * communicate with an API (send email, create calendar entry)
* separation of concerns
    * rasa core - infers what a user wants and what action to take next
    * custom actions - implement custom domain-specific actions
        * implemented in python + rasa SDK (or any other backend but requires custom code)
* rasa sdk
        * `Action` - base class to define actions
            * `name` - method which returns name we refer to in `domain.yml`
            * `run` - method which runs if the action is triggered
                * `dispatcher` - allows to send message back to user
                * `tracker` - parameter which gives access to intents, entities, and slots
                * `domain` - data defined in `domain.yml`
                * `return` is used to set events (e.g. `SlotSet`)
* build custom action
    * define intent in `nlu.yml`
    * assosciate action with intent in `rules.yml` or `stories.yml`
    * register action and intent in `domain.yml`
    * write code for custom action
* `endpoints.yml` define endpoint of actions server
* `rasa interactive` gives view into which slots are being set

### questions

* the `tracker` contains information such as `intent`, `entites`, and `slots`
* the `dispatcher` can be used to issue reponses to the user
* slots can be set by returning a `SlotSet("slot_name", value)` event

## lesson 10: basic forms

* forms can be used to collect information from a user (slot filling)
* useful if user has to give multiple pieces of information and needs to be flexible in how the user might say them
* active form can be thought of a loop, which keeps asking for information until all the slots are filled in -> then form becomes inactive -> dialog manager predicts next action
* custom actions can be used to validate form inputs
* create rule to activate form 
    * e.g. `action: simple_pizza_form` & `active_loop: simple_pizza_form`
    * 


