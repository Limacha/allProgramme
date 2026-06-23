//
//region Constants -----------------------------------------------------
//

const Attributes = Tables.get('attributes');
const Skills = Tables.get('skills');
const Classes = Tables.get('classes');
const Races = Tables.get('races');
const Bonuses = Tables.get('bonus');
const CharacterProperties = Tables.get("character_property");
const SpellLevels = Tables.get("spell-levels");
const SpellSchools = Tables.get("spell-schools");
const SpellAttacks = Tables.get("spell-attacks");
const ObjectTypes = Tables.get("object-types")
const Currencies = Tables.get("currencies");
const WeightUnits = Tables.get("units_weight");
const DistanceUnits = Tables.get("units_distance");
const ErrorMessages = Tables.get("error-messages");
const MonsterProperties = Tables.get("monster_property");
const Sizes = Tables.get("sizes");
const MonsterTypes = Tables.get("monster-types");
const Alignments = Tables.get("alignments");
const ProficiencyIcons = Tables.get("proficiency");

const MAX_LEVEL = 20;
const BASE_ARMOR_CLASS = 10;
const BASE_SPELL_SAVE_DC = 8;
const MAX_LEGENDARY_ICONS = 5;
const PASSIVE_SKILLS = {
    "perception":{
        fieldID: "passivePerception",
        propertyName: "Passive "+ Skills.get("perception").name
    },
    "investigation":{
        fieldID: "passiveInvestigation",
        propertyName: "Passive "+ Skills.get("investigation").name
    }, 
    "insight":{
        fieldID: "passiveInsight",
        propertyName: "Passive "+ Skills.get("insight").name
    }
};

const PROPERTIES_IDS = {
    main: {
        repeaterId: "characterProperties",
        table: CharacterProperties,
        choiceId: "characterPropertyChoice",
        nameId: "characterPropertyName",
        valueId: "characterPropertyDescription"
    },
    monster: {
        repeaterId: "properties",
        table: MonsterProperties,
        choiceId: "property",
        nameId: "propertyName",
        valueId: "propertyValue"
    },
    npc: {
        repeaterId: "properties",
        table: MonsterProperties,
        choiceId: "property",
        nameId: "propertyName",
        valueId: "propertyValue"
    }
}

const MONSTER_EQUIVALENT_IDS = {
    specialAbility: "special_ability",
    damageVulnerability: "damage_vulnerability",
    damageResistance: "damage_resistance",
    damageImmunity: "damage_immunity",
    conditionImmunity: "condition_immunity"
}

let OLD_NPC_IDS = {
    properties: "characterProperties",
    MonsterProperties: CharacterProperties,
    property: "characterPropertyChoice",
    propertyName: "characterPropertyName",
    propertyValue: "characterPropertyDescription",
    actions: "kornxidp"
};

const sheetsForGmTracking = [];
const gmTrackingSheets = [];
//endregion Constants
//
//
// region LET'S ROLE CORE FUNCTIONS -----------------------------------

init = function (sheet) {
    log("Initializing " + sheet.name() + " - " + sheet.properName())

    if (sheet.id() === "main") {
        initMain(sheet);
    } else if (sheet.id() === "monster" || sheet.id() === "npc") {
        initMonster(sheet);
    } else if (sheet.id() === "gmTrackingSheet") {
        //initGMTrackingSheet(sheet);
    }
};

drop = function (from, to) {
    if (from.id() === 'spell' && to.id() === "main") {
        return "spells";
    } else if (from.id() === "repeaterItem_edit" && to.id() === "main"){
        return "items";
    }
};

getCriticalHits = function (result) {
    return {
        "20": {
            "critical": [20],
            "fumble": [1]
        }
    }
};

getBarAttributes = function (sheet) {
    let stats = {}

    if (sheet.id() === "main") {
        stats[_("HP")] = ["hp", "hpmax"];
        stats[_("Hit Dice")] = ["hitdiceCurrent", "hitdiceMax"];
        stats[_("Spell Slots")] = ["spellLeft", "spellSlot"];
        stats[sheet.get('quickResourceName').value()] = ["quickResource", "quickResourceMax"];
        stats[sheet.get('quickResourceNameTwo').value()] = ["quickResourceTwo", "quickResourceMaxTwo"];
        stats[sheet.get('quickResourceNameTwo').value()] = ["quickResourceTwo", "quickResourceMaxTwo"];
        each(sheet.get('resources').value(), function (entryData, entryId){
            stats[entryData.quickResourceName] = [entryData.quickResource, entryData.quickResourceMax];
        });

    } else if (sheet.id() === "monster" || sheet.id() === "npc") {
        stats[_("HP")] = ["hp", "hpmax"];
    }

    return stats;
};

//endregion
//
//
//region MONSTER & NPC SHEET SPECIFIC FUNCTIONS ------------------------------------

const initMonster = function (sheet) {

    log("--- Transferring data to the new fields.")
    log("       NPC Properties");
    transferNPCPropertiesToMonsterFormat(sheet);
    log("       Challenge Rating");
    transferMonsterCRFromRepeater(sheet);
    log("       Speed Input");
    transferNPCMonsterSpeedInput(sheet);
    log("       Actions");
    transferNPCActionsToMonsterFormat(sheet);

    log("--- Header Summary");
    initMonsterSummary(sheet);

    log("--- HP Management")
    initHPManagement(sheet);
    initHPMaxRoll(sheet);

    log("--- Armor Class");
    initArmorClass(sheet);
    initArmorDescription(sheet);

    log("--- Attributes");
    initAttributes(sheet);

    log("--- Saves");
    initSaves(sheet);

    log("--- Initiative");
    initInitiative(sheet);

    log("--- Speed");
    initSpeed(sheet);

    log("--- Properties");
    initProperties(sheet);
    initMonsterGeneralTab(sheet);

    log("--- Passive Perception");
    initPassiveSkill(sheet, "perception");

    log("--- Attacks");
    initMonsterNPCAttacks(sheet);
    initToggleDetails(sheet);

    log("--- Tutorial");
    initTutorialPrompt(sheet);

    //log("--- Sending info to GM tracking sheet")
    //initTrackingData(sheet);

    sheet.get("initError_row").hide();
    log("Done initializing.");
};

const initMonsterSummary = function (sheet) {
    let monsterSummaryFieldList = ["monsterType", "monsterSubtype", "alignment", "challengeRating"];

    initMonsterSubtype(sheet);
    setMonsterSizeTooltip(sheet);
    setMonsterSummary(sheet);

    sheet.get('size').on('update', function () {
        setMonsterSizeTooltip(sheet);
        setMonsterSummary(sheet)
    });

    monsterSummaryFieldList.forEach(function (field) {
        sheet.get(field).on('update', function () {
            setMonsterSummary(sheet);
        });
    });
};

const setMonsterSummary = function (sheet) {
    let size = sheet.get('size').value() == "default" ? "" : _(Sizes.get(sheet.get('size').value()).name);
    let monsterType = sheet.get('monsterType').value() == "default" ? "" : _(MonsterTypes.get(sheet.get('monsterType').value()).name);
    let monsterSubtype = sheet.get('monsterSubtype').value();
    let alignment = sheet.get('alignment').value() == "default" ? "" : _(Alignments.get(sheet.get('alignment').value()).name);
    let challengeRating = sheet.get('challengeRating').value();
    let hasMonsterSummary = size || monsterType || alignment;

    if (hasMonsterSummary || challengeRating) {
        sheet.get("monsterSummary_row").show();

        if (hasMonsterSummary) {
            sheet.get('monsterSummary').show();
            let summary = _("%size • %monsterType (%monsterSubtype) • %alignment")
                .replace("%size", size)
                .replace("%monsterType", monsterType)
                .replace("%monsterSubtype", monsterSubtype)
                .replace("%alignment", alignment)
                .trim()
                .replace("()", "") //remove empty brackets if no monsterSubtype
                .replace("  ", " ")
                .replace(" • • ", " ")
                .replace(/^•/, "") //remove dot at beginning of sentence if it occurs
                .replace(/•$/, ""); //remove dot at end of sentence if it occurs
            sheet.get('monsterSummary').text(summary);
        } else sheet.get('monsterSummary').hide();

        if (challengeRating) {
            sheet.get('challengeRating_row').removeClass("invisible");
            sheet.get('challengeRating_read').text(challengeRating);
        } else sheet.get('challengeRating_row').addClass("invisible");

    } else {
        sheet.get("monsterSummary_row").hide();
    }
};

const setMonsterSizeTooltip = function (sheet) {
    let size = sheet.get('size').value() || "default";
    let tooltipLabel = "";

    if (size != "default") {
        let sizeLabel = _(Sizes.get(size).spaceLabel);
        tooltipLabel = _("Controls a space of %size").replace("%size", sizeLabel);
    }

    sheet.get("size").setToolTip(tooltipLabel);
};

const initMonsterSubtype = function (sheet) {
    let addSubtype = function () {
        sheet.get("monsterSubtype_row").show();
        sheet.get("monsterSubtype_remove").show();
        sheet.get("monsterSubtype_add").hide();
    }
    let removeSubtype = function () {
        sheet.get("monsterSubtype_row").hide();
        sheet.get("monsterSubtype_remove").hide();
        sheet.get("monsterSubtype_add").show();
        sheet.get("monsterSubtype").value("");
    }

    let hasSubtypeProvided = function () {
        return Boolean(sheet.get("monsterSubtype").value());
    }

    if (hasSubtypeProvided) addSubtype();
    sheet.get('monsterSubtype_add').on("click", addSubtype);
    sheet.get('monsterSubtype_remove').on("click", removeSubtype);
};

const initHPMaxRoll = function (sheet) {
    const setButtonState = function () {
        let hpRoll = sheet.get('hpmax_roll').value();
        if (hpRoll)
            sheet.get('hpmax_rollBtn').removeClass("disabled").addClass("shadow-sm");
        else sheet.get('hpmax_rollBtn').addClass("disabled").removeClass("shadow-sm");
    }
    const rollHP = function () {
        let hpRoll = sheet.get('hpmax_roll').value();
        if (hpRoll) {
            let diceRoll = Dice.create(hpRoll);
            let sheetName = sheet.properName();
            let rollTitle = _("Rolling Max HP for %sheetName").replace("%sheetName", sheetName);
            let rollAction = {};
            rollAction[_("Use as HP Max")] = function (dice) {
                if (sheet.get("hp").value() > dice.total || sheet.get("hp").value() == sheet.get("hpmax").value())
                    sheet.get("hp").value(dice.total);
                sheet.get("hpmax").value(dice.total);
            }
            Dice.roll(sheet, diceRoll, rollTitle, "gmonly", rollAction);
        }
    }
    setButtonState();
    sheet.get('hpmax_roll').on('update', setButtonState);
    sheet.get('hpmax_rollBtn').on('click', rollHP);
};

const initArmorDescription = function (sheet) {
    const setArmorDescriptionTooltip = function () {
        let armorDescription = sheet.get('armorDescription').value();
        sheet.get('ac').setToolTip(armorDescription, "bottom");
    };

    setArmorDescriptionTooltip();
    sheet.get('armorDescription').on('update', setArmorDescriptionTooltip);
};

const initMonsterGeneralTab = function (sheet) {

    initToggleEdit(sheet, "properties");

    const rollMonsterSkill = function (target) {
        let entry = sheet.get('skills').find(target.index());
        let skillName = entry.find("propertyName").value() || "";
        if (skillName) {
            let skill = getTableElementFromName(Skills, skillName);
            if (skill)
                rollSkill(sheet, skill);
        }
    };

    sheet.get('skills').on('click', 'nameLabel', rollMonsterSkill);
    setMonsterGeneralTab(sheet);
};

const setMonsterGeneralTab = function (sheet) {
    setMonsterSkills(sheet);
    setMonsterSpecialAbilities(sheet);
    setMonsterSenses(sheet);
    setMonsterVulnerabilitiesAndResistances(sheet);
    setMonsterLanguages(sheet);
};

const setMonsterSkills = function (sheet){
    let monsterSkills = getSelectProperties(sheet, "skill");
    if(!objectsEqual(sheet.get('skills').value(), monsterSkills))
        sheet.get('skills').value(monsterSkills);
}

const setMonsterSenses = function (sheet) {
    let sensesLabel = ""
    let senses = getSelectProperties(sheet, "sense");

    for (entryId in senses) {
        let senseName = getPropertyTitle(sheet, entryId) || "";
        let senseValue = getPropertyValue(sheet, entryId) || ""

        let isPassivePerception = trim(senseName) == trim("Passive Perception") || trim(senseName) == trim(_("Passive Perception"));

        if (!isPassivePerception && (senseName || senseValue)) {
            if (sensesLabel)
                sensesLabel += ", ";
            sensesLabel += senseName;
            if (senseName && senseValue)
                sensesLabel += " ";
            sensesLabel += senseValue;
        }
    }

    if (sensesLabel) {
        sheet.get('sensesRead').removeClass("font-italic").removeClass("opacity-50");
        sheet.get('sensesRead').text(sensesLabel);
    } else {
        sheet.get('sensesRead').addClass("font-italic").addClass("opacity-50");
        sheet.get('sensesRead').text(_("No additional senses."));
    }
};

const setMonsterVulnerabilitiesAndResistances = function (sheet) {
    let propertiesList = ["damageVulnerability", "damageResistance", "damageImmunity", "conditionImmunity"];
    let hasAtLeastOne = false;

    propertiesList.forEach(function (propertyType) {
        let properties = getSelectProperties(sheet, propertyType);
        if (Object.keys(properties).length > 0) hasAtLeastOne = true;

        let tempLabel = ""
        for (entryId in properties) {
            let propertyName = getPropertyTitle(sheet, entryId) || "";
            let propertyValue = getPropertyValue(sheet, entryId) || "";

            if (propertyName || propertyValue) {
                if (tempLabel)
                    tempLabel += ", ";
                tempLabel += _(propertyName);
                if (propertyName && propertyValue)
                    tempLabel += " ";
                tempLabel += _(propertyValue);
            }
        }

        if (tempLabel) {
            sheet.get(propertyType + "Read_row").show();
            sheet.get(propertyType + "Read").text(tempLabel);
        } else {
            sheet.get(propertyType + "Read_row").hide();
        }
    })

    if (hasAtLeastOne) {
        sheet.get('resistances_none').hide();
    } else {
        sheet.get('resistances_none').show();
    }
};

const setMonsterLanguages = function (sheet) {
    let languagesLabel = ""
    let languages = getSelectProperties(sheet, "language");

    for (entryId in languages) {
        let languageName = getPropertyTitle(sheet, entryId) || "";
        let languageValue = getPropertyValue(sheet, entryId) || ""

        if (languageName || languageValue) {
            if (languagesLabel)
                languagesLabel += ", ";
            languagesLabel += languageName;
            if (languageName && languageValue)
                languagesLabel += " ";
            languagesLabel += languageValue;
        }
    }

    if (languagesLabel) {
        sheet.get('languagesRead').removeClass("font-italic").removeClass("opacity-50");
        sheet.get('languagesRead').text(languagesLabel);
    } else {
        sheet.get('languagesRead').addClass("font-italic").addClass("opacity-50");
        sheet.get('languagesRead').text(_("No additional languages."));
    }
};

const setMonsterSpecialAbilities = function (sheet) {
    let specialAbilities = getSelectProperties(sheet, "specialAbility");

    if (Object.keys(specialAbilities).length > 0) {
        sheet.get('specialAbilities_title').show();
        sheet.get('specialAbilities').show();
        let newAbilities = getSelectProperties(sheet, "specialAbility");
        if(!objectsEqual(sheet.get('specialAbilities').value(), newAbilities))
            sheet.get('specialAbilities').value(newAbilities);

        each(sheet.get('specialAbilities').value(), function (entryData, entryId) {
            sheet.get('specialAbilities').find(entryId).find("propertyDisplay").hide();
        })
    }
    else {
        sheet.get('specialAbilities_title').hide();
        sheet.get('specialAbilities').hide();
    }
};

const initMonsterNPCAttacks = function (sheet) {
    const attackRepeaterIds = ["actions", "actionsRead_common", "actionsRead_legendary"];

    const rollAction = function (repeaterId, target) {
        let entryData = sheet.get(repeaterId).value()[target.index()];

        if (entryData.attackHit) {
            rollAttack(sheet, entryData.attackName, entryData.attackHit, entryData.attackDamage)
        } else if (entryData.attackDamage) {
            rollDamages(sheet, entryData.attackName, entryData.attackDamage, false)
        }
    }

    const setReadRepeatersData = function () {

        const promoteMultiattackToTop = function (obj) {
            const sortedKeys = Object.keys(obj).sort(function (a, b) {
                let entryA = obj[a];
                let entryB = obj[b];
                if (trim(entryA.attackName).includes(trim(_("Multiattack"))))
                    return -1;
                if (trim(entryB.attackName).includes(trim(_("Multiattack"))))
                    return 1;
                else return 0;
            });

            const sortedObj = sortedKeys.reduce(function (result, key) {
                result[key] = obj[key];
                return result;
            }, {});
            return sortedObj;
        }

        let actionsRead_common_new = {};
        let actionsRead_legendary_new = {};

        each(sheet.get("actions").value(), function (entryData, entryId) {
            if (entryData.legendary)
                actionsRead_legendary_new[entryId] = entryData;
            else actionsRead_common_new[entryId] = entryData;
        });

        actionsRead_common_new = promoteMultiattackToTop(actionsRead_common_new);
        actionsRead_legendary_new = promoteMultiattackToTop(actionsRead_legendary_new);
        if(!objectsEqual(sheet.get("actionsRead_common").value(),actionsRead_common_new))
            sheet.get("actionsRead_common").value(actionsRead_common_new);
        if(!objectsEqual(sheet.get("actionsRead_legendary").value(), actionsRead_legendary_new))
            sheet.get("actionsRead_legendary").value(actionsRead_legendary_new);
    }

    const showCommonActionsWarning = function() {
        let actionsRead_common = sheet.get('actionsRead_common').value();
        if (actionsRead_common && Object.keys(actionsRead_common).length > 0) {
            sheet.get('warning_clickToAdd').hide();
        } else sheet.get('warning_clickToAdd').show();
    }

    const refreshRepeaterDisplay = function (target) {
        let repeaterId = (typeof target == "string") ? target : target.id();
        let repeater = sheet.get(repeaterId);

        each(repeater.value(), function (entryData, entryId) {
            let attackSavingThrowLabel;
            let entry = repeater.find(entryId);

            if (entryData.attackSt && entryData.attackSt != "0") {
                attackSavingThrowLabel = _(Attributes.get(entryData.attackSt).name);
                entry.find("attackSavingThrowLabel").text(attackSavingThrowLabel);
            } else entry.find("attackSavingThrowRow").hide();

            if (!entryData.attackHit)
                entry.find("toHitRow").hide();
            else entry.find("toHitRow").show();

            if (!entryData.attackType)
                entry.find("damageTypeRow").hide();
            else entry.find("damageTypeRow").show();

            if (!entryData.attackDamage)
                entry.find("damageRow").hide();
            else entry.find("damageRow").show();

            if (!entryData.attackDamage && !entryData.attackType)
                entry.find("nameLabel").removeClass("clickable");
        })

    }

    const refreshAllDisplays = function () {
        setReadRepeatersData();
        each(attackRepeaterIds, function (repeater) {
            refreshRepeaterDisplay(repeater);
        })
        showCommonActionsWarning();
        toggleLegendaryActionsRow(sheet);
    }

    each(attackRepeaterIds, function (repeater) {
        sheet.get(repeater).on("click", "nameLabel", function (target) {
            rollAction(repeater, target);
        });
    });

    refreshAllDisplays();
    sheet.get("actions").on("update", refreshAllDisplays);

    initLegendaryActionsCounter(sheet);
    initLegendaryResistancesCounter(sheet);
    initToggleEdit(sheet, "actions");
};

const initLegendaryActionsCounter = function (sheet) {

    const getCounterLimit = function () {
        let counter = parseInt(sheet.get("maxLegendaryActions").value()) || 3;
        return counter;
    }

    const getLegendaryActionsTracker = function () {
        let tracker = [];
        let counterLimit = getCounterLimit();

        //IMPORT STORED TRACKER IF APPLICABLE
        let hasLegendaryActionsTracker = sheet.getData().hasOwnProperty("legendaryActionsTracker");
        if (hasLegendaryActionsTracker)
            tracker = sheet.getData()["legendaryActionsTracker"];

        //CHECK & ADJUST LENGTH
        let currentLength = tracker.length;
        if (currentLength > counterLimit) {
            tracker.splice(counterLimit)
        }
        else for (let j = 0; j < counterLimit - currentLength; j++) {
            tracker.push(true);
        }

        return tracker;
    }

    const displayLegendaryActions = function () {
        let tracker = getLegendaryActionsTracker();

        for (let i = 0; i < tracker.length; i++) {
            let icon = sheet.get('legendaryAction_' + i);
            icon.show();
            if (tracker[i])
                icon.removeClass("opacity-25");
            else icon.addClass("opacity-25");
        }
        for (let i = tracker.length; i < MAX_LEGENDARY_ICONS; i++) {
            let icon = sheet.get('legendaryAction_' + i);
            icon.hide();
        }
        let labelText = _("The monster can take %maxLegendaryActions legendary actions, choosing from the options below. Only one legendary action option can be used at a time and only at the end of another creature's turn. The monster regains spent legendary actions at the start of its turn.")
            .replace("%maxLegendaryActions", tracker.length)
        sheet.get("legendaryActions_label").text(labelText);
    }

    const resetLegendaryActions = function () {
        let tracker = [];
        let counterLimit = getCounterLimit();
        for (let i = 0; i < counterLimit; i++) {
            tracker.push(true);
        }
        sheet.setData({ "legendaryActionsTracker": tracker });
        displayLegendaryActions();
    }

    const toggleLegendaryActionStatus = function (target) {
        let i = target.id().slice(-1);
        let tracker = getLegendaryActionsTracker();

        if (i < tracker.length) {
            tracker[i] = !tracker[i];
            sheet.setData({ "legendaryActionsTracker": tracker });
            displayLegendaryActions();
        }
    }

    toggleLegendaryActionsRow(sheet);
    displayLegendaryActions();
    for (let i = 0; i < MAX_LEGENDARY_ICONS; i++) {
        sheet.get('legendaryAction_' + i).on('click', toggleLegendaryActionStatus);
    }
    sheet.get("resetLegendaryActions").on('click', resetLegendaryActions);
    sheet.get("maxLegendaryActions").on('update', function(){
        toggleLegendaryActionsRow(sheet);
        displayLegendaryActions();
    });
}

const toggleLegendaryActionsRow = function(sheet){
    let legendaryActions = sheet.get('actionsRead_legendary').value() || {};
    let hasLegendaryActions = Object.keys(legendaryActions).length || sheet.get("maxLegendaryActions").value();
    if (hasLegendaryActions) {
        sheet.get('legendaryActions_row').show();
    } else sheet.get('legendaryActions_row').hide();
}

const initLegendaryResistancesCounter = function (sheet) {

    const getCounterLimit = function () {
        let counter = parseInt(sheet.get("maxLegendaryResistances").value()) || 3;
        return counter;
    }

    const getLegendaryResistancesTracker = function () {
        let tracker = [];
        let counterLimit = getCounterLimit();

        //IMPORT STORED TRACKER IF APPLICABLE
        let hasLegendaryResistancesTracker = sheet.getData().hasOwnProperty("LegendaryResistancesTracker");
        if (hasLegendaryResistancesTracker)
            tracker = sheet.getData()["LegendaryResistancesTracker"];

        //CHECK & ADJUST LENGTH
        let currentLength = tracker.length;
        if (currentLength > counterLimit) {
            tracker.splice(counterLimit)
        }
        else for (let j = 0; j < counterLimit - currentLength; j++) {
            tracker.push(true);
        }

        return tracker;
    }

    const displayLegendaryResistances = function () {
        let tracker = getLegendaryResistancesTracker();

        for (let i = 0; i < tracker.length; i++) {
            let icon = sheet.get('legendaryResistance_' + i);
            icon.show();
            if (tracker[i])
                icon.removeClass("opacity-25");
            else icon.addClass("opacity-25");
        }
        for (let i = tracker.length; i < MAX_LEGENDARY_ICONS; i++) {
            let icon = sheet.get('legendaryResistance_' + i);
            icon.hide();
        }
    }

    const resetLegendaryResistances = function () {
        let tracker = [];
        let counterLimit = getCounterLimit();
        for (let i = 0; i < counterLimit; i++) {
            tracker.push(true);
        }
        sheet.setData({ "LegendaryResistancesTracker": tracker });
        displayLegendaryResistances();
    }

    const toggleLegendaryResistancestatus = function (target) {
        let i = target.id().slice(-1);
        let tracker = getLegendaryResistancesTracker();

        if (i < tracker.length) {
            tracker[i] = !tracker[i];
            sheet.setData({ "LegendaryResistancesTracker": tracker });
            displayLegendaryResistances();
        }
    }

    toggleLegendaryResistancesRow(sheet);
    displayLegendaryResistances();
    for (let i = 0; i < MAX_LEGENDARY_ICONS; i++) {
        sheet.get('legendaryResistance_' + i).on('click', toggleLegendaryResistancestatus);
    }
    sheet.get("resetLegendaryResistances").on('click', resetLegendaryResistances);
    sheet.get("maxLegendaryResistances").on('update', function () {
        toggleLegendaryResistancesRow(sheet);
        displayLegendaryResistances();
    });
};

const toggleLegendaryResistancesRow = function (sheet) {
    let hasLegendaryResistances = getPropertyIndexFromName(sheet, "specialAbility", _("Legendary Resistance"));
    if (!hasLegendaryResistances)
        hasLegendaryResistances = getPropertyIndexFromName(sheet, MONSTER_EQUIVALENT_IDS["specialAbility"], _("Legendary Resistance"));
    if (!hasLegendaryResistances)
        hasLegendaryResistances = parseInt(sheet.get("maxLegendaryResistances").value()) > 0

    if (hasLegendaryResistances) {
        sheet.get("legendaryResistances_row").show();
    } else sheet.get("legendaryResistances_row").hide();
};

const transferNPCPropertiesToMonsterFormat = function (sheet) {
    let sheetData = sheet.getData();

    let hasNpcProperties = sheetData.hasOwnProperty(OLD_NPC_IDS["properties"]) && Object.keys(sheetData[OLD_NPC_IDS["properties"]]).length > 0;
    let hasMonsterProperties = sheet.get("properties").value() && Object.keys(sheet.get("properties").value()).length > 0;

    if (hasNpcProperties && !hasMonsterProperties) {
        let currentProperties = sheetData[OLD_NPC_IDS["properties"]];
        let newProperties = {};

        for (entryId in currentProperties) {
            let entryData = currentProperties[entryId];
            let newEntryData = {};
            for (field in entryData) {
                if (field === OLD_NPC_IDS["propertyName"]) {
                    newEntryData["propertyName"] = entryData[OLD_NPC_IDS["propertyName"]];
                } else if (field === OLD_NPC_IDS["propertyValue"]) {
                    newEntryData["propertyValue"] = entryData[OLD_NPC_IDS["propertyValue"]];
                } else if (field === OLD_NPC_IDS["property"]) {
                    let equivalentId = MONSTER_EQUIVALENT_IDS[entryData[field]] || entryData[field];
                    let newChoice = "default";
                    if(isValidTableElement(MonsterProperties, equivalentId))
                        newChoice = MonsterProperties.get(equivalentId).id;
                    newEntryData["property"] = newChoice;
                } else newEntryData[field] = entryData[field];
            }
            newProperties[entryId] = newEntryData;
        }

        //let dataToSet = {}
        //dataToSet["properties"] = newProperties;
        //dataToSet[oldNPCIds["properties"]] = {};
        //sheet.setData(dataToSet);
        sheet.get('properties').value(newProperties);
    }
};

const transferNPCMonsterSpeedInput = function (sheet) {
    let speedInput = sheet.getData()["speedInput"];

    if (speedInput) {
        let properties = sheet.get("properties").value();

        let newProperties = {};
        for (entryId in properties) {
            newProperties[entryId] = properties[entryId];
        }

        if(!newProperties.hasOwnProperty("transferredSpeedInput")) {
            newProperties["transferredSpeedInput"] = {
                property: "speed",
                propertyValue: speedInput
            };
        }

        sheet.get("properties").value(newProperties);
        //sheet.setData({ "speedInput": 0 });
    }
};

const transferNPCActionsToMonsterFormat = function (sheet) {
    let sheetData = sheet.getData();

    let hasNpcActions = sheetData.hasOwnProperty(OLD_NPC_IDS["actions"]) && Object.keys(sheetData[OLD_NPC_IDS["actions"]]).length > 0;
    let hasMonsterActions = sheet.get("actions").value() && Object.keys(sheet.get("actions").value()).length > 0;

    if (hasNpcActions && !hasMonsterActions) {
        let currentActions = sheetData[OLD_NPC_IDS["actions"]];
        let newActions = {};

        for (entryId in currentActions) {
            newActions[entryId] = currentActions[entryId];
        }

        //let dataToSet = {};
        //dataToSet["actions"] = newActions;
        //dataToSet[oldNPCIds["actions"]] = {};

        //sheet.setData(dataToSet);
        sheet.get('actions').value(newActions);
    }
};

const transferMonsterCRFromRepeater = function (sheet) {

    let foundChallengeRating = false;
    let challengeRating = "";

    let properties = sheet.get("properties").value();
    let newProperties = {};

    for (entryId in properties) {
        let entryData = properties[entryId];
        if (entryData.property && entryData.property == "challenge_rating") {
            if (!foundChallengeRating || !challengeRating) {
                foundChallengeRating = true;

                challengeRating = entryData.propertyName || "";
                if (entryData.propertyValue)
                    challengeRating += challengeRating ? " (" + entryData.propertyValue + ")" : entryData.propertyValue;
            }
        } else {
            newProperties[entryId] = entryData;
        }
    }

    if (foundChallengeRating) {
        if (!sheet.get('challengeRating').value()) {
            sheet.get('challengeRating').value(challengeRating);
        }
        //sheet.setData({"properties": newProperties});
    }
};

//endregion MONSTER & NPC SHEET SPECIFIC FUNCTIONS
//

//
//region GM Tracking Sheet
//

const initGMTrackingSheet = function (sheet) {
    log("Initializing GM tracking sheet")

    initTrackingData(sheet);
    initMassInitiativeRoll(sheet);
};

const initTrackingData = function (sheet) {
    if (sheet.id() === "gmTrackingSheet" && gmTrackingSheets.length == 0) {
        gmTrackingSheets.push(sheet);
        setTrackingData("main");
        setTrackingData("npc");
        setTrackingData("monster");

    } else if (sheet.id() === "main" || sheet.id() === "npc" || sheet.id() === "monster") {
        sheetsForGmTracking.push(sheet);
        setTrackingData(sheet.id());
    }
}

const setTrackingData = function (sheetType) {
    if (gmTrackingSheets.length > 0 && sheetsForGmTracking.length > 0) {
        let trackingSheet = gmTrackingSheets[0];
        let repeater = trackingSheet.get(sheetType + "Sheets");

        const refreshTrackerDisplay = function () {
            if (sheetType === "main") {
                each(repeater.value(), function (entryData, entryId) {
                    let entry = repeater.find(entryId);
                    if (entryData.inspiration)
                        entry.find("inspiration_icon").removeClass("opacity-25");
                    else entry.find("inspiration_icon").addClass("opacity-25")

                    for (let i = 1; i <= 3; i++) {
                        if (entryData["deathSaveSuccess" + i] == "circle")
                            entry.find("deathSaveSuccess" + i).addClass("opacity-25");
                        else entry.find("deathSaveSuccess" + i).removeClass("opacity-25");

                        if (entryData["deathSaveFailure" + i] == "circle")
                            entry.find("deathSaveFailure" + i).addClass("opacity-25");
                        else entry.find("deathSaveFailure" + i).removeClass("opacity-25");
                    }
                })
            } else {
                each(repeater.value(), function (entryData, entryId) {
                    let entry = repeater.find(entryId);
                    entry.find("inspiration_col").addClass("d-none");
                    entry.find("deathSaves_col").addClass("d-none");
                    entry.find('speed').removeClass("text-medium");
                    entry.find("speedCol").removeClass("w-50px").removeClass("text-center");
                })
            }
        }

        let newRepeaterValue = {};
        each(sheetsForGmTracking, function (sheet) {
            if (sheet.id() === sheetType) {
                let sheetID = sheet.getSheetId().toString();
                newRepeaterValue[sheetID] = {
                    "name": sheet.properName(),
                    "hp": getHPCurrent(sheet),
                    "hpmax": getHPMax(sheet),
                    "tempHPLabel": sheet.get('tempHP').value() > 0 ? ("+" + sheet.get('tempHP').value()) : "",
                    "ac": getArmorClass(sheet),
                    "speed": getSpeed(sheet),
                    "passivePerception": getPassivePerception(sheet)
                }
                if (sheetType === "main") {
                    newRepeaterValue[sheetID]["inspiration"] = sheet.get("inspiration").value();
                    for (let i = 1; i <= 3; i++) {
                        newRepeaterValue[sheetID]["deathSaveSuccess" + i] = sheet.get("deathSaveSuccess" + i).value() ? "check-circle" : "circle";
                        newRepeaterValue[sheetID]["deathSaveFailure" + i] = sheet.get("deathSaveFailure" + i).value() ? "times-circle" : "circle";
                    }
                }
            }
        })
        repeater.value(newRepeaterValue);
        refreshTrackerDisplay();
    }
}

const initMassInitiativeRoll = function (sheet) {
    const rollMassInitiative = function () {
        each(sheetsForGmTracking, function (entitySheet) {
            rollInitiative(entitySheet);
        })
    }
    sheet.get('massInitiative_btn').on('click', rollMassInitiative);
}

//endregion
//

//
// Initialization
//

const initMain = function (sheet) {
    log("---Races");
    initCustomRaces(sheet);

    log("---Classes, including Multiclass");
    initClasses(sheet);    

    log("---Character Level")
    initCharacterLevel(sheet);

    log("---Inspiration");
    initInspiration(sheet);

    log("---Proficiency Bonus");
    initProficiencyBonus(sheet);

    log("---Attributes");
    initAttributes(sheet);

    log("---Health Management");
    initHPManagement(sheet);

    log("---Saves");
    initSaves(sheet);

    log("---Skills");
    initSkills(sheet);

    log("---Passive Skills");
    initPassiveSkills(sheet);

    log("---Armor Class");
    initArmorClass(sheet);

    log("---Initiative");
    initInitiative(sheet);

    log("---Speed");
    initSpeed(sheet);

    log("---Quick Resources");
    initQuickResources(sheet);

    log("---Long & Short Rest");
    initLongRest(sheet);
    initShortRest(sheet);

    log("---Weapons");
    initWeapons(sheet);

    log("---Spells");
    initSpells(sheet);

    log("---Items");
    initItems(sheet);

    log("---Features");
    initFeatures(sheet);
    log("bonuses and expertises"); initBonusAndExpertise(sheet);

    log("---Character Properties");
    initProperties(sheet);

    log("---Settings Tab");
    initSettings(sheet);

    log("--- Toggle Details");
    initToggleDetails(sheet);

    //log("--- Sending info to GM tracking sheet")
    //initTrackingData(sheet);
    sheet.get("initError_row").hide();
    log("Initialization completed. Have fun!")
};

const initCustomRaces = function (sheet) {
    setRaceChoices(sheet);

    sheet.get('customRacesEdit').on('click', function (){
        sheet.prompt(_("Manage Custom Races"), "promptCustomRaces", function (promptData) {
                sheet.setData({"customRaces" : promptData.customRaces})
                setRaceChoices(sheet);
            }, function(promptView) {
                promptView.get("customRaces").value(sheet.get('customRaces').value());
        });
    });
};

const setRaceChoices = function (sheet) {
    const sortRaces = function(races){
        
        const compareByName = function(a,b){
            if(a == "null" || _(races[a]).toLowerCase() < _(races[b]).toLowerCase()){
                return -1;
            } else if(b == "null" || _(races[a]).toLowerCase() > _(races[b]).toLowerCase()){
                return 1;
            } else {
                return 0;
            }
        }
        let objectKeys = Object.keys(races);
        objectKeys.sort(compareByName);

        let sortedRaces = {};
        objectKeys.forEach(function(entryId){
            sortedRaces[entryId] = races[entryId];
        });
        return sortedRaces;
    }
    let selectedRace = sheet.get("race").value();
    let races = sortRaces(getRaces(sheet));
    let selectedRaceIsValid = races.hasOwnProperty(selectedRace);

    sheet.get("race").setChoices(races);
    if(!selectedRaceIsValid) sheet.get("race").value("null");
};

const initClasses = function (sheet) {
    log("----- Custom Classes");
    initCustomClasses(sheet);
    log("----- Multiclass");
    initMulticlass(sheet);
    log("----- Hit Dice")
    setHitDice(sheet);
};

const initMulticlass = function (sheet) {

    const addNewClass = function() {
        let currentMultiClassMore = sheet.get("multiClassMore").value() || {};
        let newMultiClassMore = {}; 
        for(entryId in currentMultiClassMore){
            newMultiClassMore[entryId] = currentMultiClassMore[entryId];
        }

        let currentCount = Object.keys(newMultiClassMore).length;
        let newMultiClass = {};
        newMultiClass["multiClass_"+currentCount+1] = {
            "class": "null",
            "level": 1
        };
        Object.assign(newMultiClassMore, newMultiClass);
        sheet.get("multiClassMore").value(newMultiClassMore);
    };

    const deleteClass = function(target) {
        let currentMultiClassMore = sheet.get("multiClassMore").value() || {};
        let newMultiClassMore = {};
        for(entryId in currentMultiClassMore){
            if(entryId !== target.index())
                newMultiClassMore[entryId] = currentMultiClassMore[entryId]
        }
        sheet.get("multiClassMore").value(newMultiClassMore);
        displayMulticlassComponents();
    }

    const displayMulticlassComponents = function () {
        log("sheet.get('multiClassLevel').value(): " + sheet.get('multiClassLevel').value())
        log("sheet.get('multiClassChoice').value(): " + sheet.get('multiClassChoice').value())
        let hasFirstMulticlass = sheet.get('multiClassLevel').value() || sheet.get('multiClassChoice').value() != "null";
        let hasMoreMultiClass = sheet.get("multiClassMore").value() && Object.keys(sheet.get("multiClassMore").value()).length;
        if (hasFirstMulticlass)
            sheet.get("multiclassRow").show();
        else 
            sheet.get("multiclassRow").hide();
        if (hasMoreMultiClass)
            sheet.get("multiClassMore").show();
        else sheet.get("multiClassMore").hide();
    };

    displayMulticlassComponents();

    sheet.get("multiclassDeleteBtn").on('click', function(){
        sheet.get('multiClassLevel').value(0);
        sheet.get('multiClassChoice').value("null");
        sheet.get("multiclassRow").hide();
    });

    sheet.get("multiclassAdd").on('click', function(){
        addNewClass();
        displayMulticlassComponents();
    });

    sheet.get("multiClassMore").on('click', 'deleteBtn', deleteClass);
    sheet.get("multiClassMore").on('update', function(){
        setClassChoices(sheet);
    });

};

const initCustomClasses = function (sheet) {
    
    const setPromptView = function(promptView){
        let customClasses = promptView.get("customClasses").value() || {};
        each(customClasses, function(entryData, entryId){
            if(entryData.spellcastingAbility){
                let spellcastingAbility_read = _(Attributes.get(entryData.spellcastingAbility).name);
                promptView.get("customClasses").find(entryId).find("spellcastingAbility_read").text(spellcastingAbility_read);
            }
        });
    }

    setClassChoices(sheet);
    sheet.get('customClassesEdit').on('click', function (){
        sheet.prompt(_("Manage Custom Classes"), "promptCustomClasses", function (promptData) {
                let customClasses = promptData.customClasses || {}
                sheet.setData({"customClasses" : customClasses})
                setClassChoices(sheet);
            }, function(promptView) {
                let customClasses = sheet.get('customClasses').value() || {};
                promptView.get("customClasses").value(customClasses);
                
                setPromptView(promptView);
                promptView.get("customClasses").on('update', function(){
                    setPromptView(promptView);
                });
        });
    });
};

const setClassChoices = function (sheet) {

    const sortClasses = function(classes){
        
        const compareByName = function(a,b){
            if(classes[a].id == "null" || _(classes[a].name).toLowerCase() < _(classes[b].name).toLowerCase()){
                return -1;
            } else if(classes[b].id == "null" || _(classes[a].name).toLowerCase() > _(classes[b].name).toLowerCase()){
                return 1;
            } else {
                return 0;
            }
        }
        let objectKeys = Object.keys(classes);
        objectKeys.sort(compareByName);

        let sortedClasses = {};
        objectKeys.forEach(function(entryId){
            sortedClasses[entryId] = classes[entryId].name;
        });
        return sortedClasses;
    }

    let selectedClasses = getSelectedClasses(sheet);
    let classNames = sortClasses(getClasses(sheet));

    for (classIndex in selectedClasses){
        let thisClass = selectedClasses[classIndex];

        let classChoiceComponent;
        if(classIndex == "baseClass" || classIndex == "firstMulticlass")
            classChoiceComponent = sheet.get(thisClass.classField)
        else classChoiceComponent = sheet.get("multiClassMore").find(classIndex).find(thisClass.classField);
        
        classChoiceComponent.setChoices(classNames);

        if(!isValidClass(sheet, thisClass.id))
            classChoiceComponent.value("null");
    }
};

const initCharacterLevel = function (sheet) {
    setCharacterSummary(sheet);
    initToggleEdit(sheet, "characterSummary")
};

const setCharacterSummary = function (sheet){
    let characterSummary = "";
    let race = sheet.get('race').value();

    let selectedClasses = getSelectedClasses(sheet);
    let isFirstClass = true;

    for(classIndex in selectedClasses){
        let thisClass = selectedClasses[classIndex];
        if(isValidClass(sheet, thisClass.id) && thisClass.id != "null" && thisClass.level > 0){
            if(!isFirstClass) characterSummary += ",";
            if(characterSummary) characterSummary += " ";

            characterSummary += _(thisClass.name);
            if(isMulticlass(sheet)) characterSummary += " " + thisClass.level;
            isFirstClass = false;
        }
    }

    if(characterSummary == "")
        characterSummary = _("Click pen to select a class and level.")
    else characterSummary = _(getRaces(sheet)[race]) + " " + characterSummary;

    sheet.get("levelTotal").text(getLevel(sheet));
    sheet.get("characterSummary").text(characterSummary);

    let summaryLength = characterSummary.split('').length;
    if(summaryLength <= 32){
        sheet.get('characterSummaryReadContainer').removeClass("h-75px").addClass("h-50px");
        sheet.get("characterSummary").removeClass("text-tiny");
    } else if(summaryLength > 32 && summaryLength <= 64){
        sheet.get('characterSummaryReadContainer').removeClass("h-75px").addClass("h-50px");
        sheet.get("characterSummary").addClass("text-tiny");
    } else {
        sheet.get('characterSummaryReadContainer').addClass("h-75px").removeClass("h-50px");
        sheet.get("characterSummary").addClass("text-tiny");
    }
};

const setHitDice = function (sheet) {
    let hitDiceMore = sheet.get("hitDiceMore").value() || {};
    let selectedClasses = getSelectedClasses(sheet);
    let dataToSet = {};
    let hitDiceMore_new = {};
    for (classIndex in selectedClasses){
        let thisClass = selectedClasses[classIndex];
        let newHitDiceMax = thisClass.level;
        let hasThisClassInHitDiceMore = hitDiceMore.hasOwnProperty(classIndex);
        let initialHitDiceMax = 0;
        if(classIndex == "baseClass" || classIndex == "firstMulticlass"){
            initialHitDiceMax = sheet.get(thisClass.maxHitDiceField).value();
        } else if(hasThisClassInHitDiceMore) {
            initialHitDiceMax = sheet.get("hitDiceMore").find(classIndex).find(thisClass.maxHitDiceField).value();
        };
        let hitdiceEvol = newHitDiceMax - initialHitDiceMax;
        let initialHitDiceCurrent = thisClass.hitdiceCurrent;
        let newHitDiceCurrent = Math.min(newHitDiceMax, Math.max(initialHitDiceCurrent, initialHitDiceCurrent + hitdiceEvol));

        if(classIndex == "baseClass" || classIndex == "firstMulticlass"){
            if(hitdiceEvol != 0){
                dataToSet[thisClass.maxHitDiceField] = newHitDiceMax;
                dataToSet[thisClass.currentHitDiceField] = newHitDiceCurrent;
            }
        } else {
            hitDiceMore_new[classIndex] = {
                "hitdiceMax": newHitDiceMax,
                "hitdiceCurrent": newHitDiceCurrent
            };                    
        }
    }

    dataToSet["hitDiceMore"] = hitDiceMore_new;
    sheet.setData(dataToSet);
};

const initInspiration = function (sheet){
    const toggleInspiration = function (){
        let oppositeValue = !sheet.get('inspiration').value();
        sheet.get('inspiration').value(oppositeValue);
    };
    const refreshInspirationBtn = function(){
        if(sheet.get('inspiration').value())
            sheet.get('inspirationBtn').addClass("active").setToolTip(_("Click to spend Heroic Inspiration."), "right");
        else sheet.get('inspirationBtn').removeClass("active").setToolTip(_("Click to mark Heroic Inspiration."), "right");
    }
    refreshInspirationBtn();
    sheet.get('inspiration').on('update', refreshInspirationBtn);
    sheet.get('inspirationBtn').on('click', toggleInspiration);
};

const initProficiencyBonus = function (sheet) {
    refreshProficiencyBonusLabel(sheet);

    sheet.get("proficiencyInput").on("update", function () {
        setProficiencyBonus(sheet);
    })
    initToggleEdit(sheet, "proficiencyBonus");
};

const setProficiencyBonus = function (sheet) {
    refreshProficiencyBonusLabel(sheet);
    
    setSkillLabels(sheet);
    setSaveLabels(sheet);
    setSpellSaveDCLabel(sheet);
    setSpellAttackBonusLabel(sheet);
};

const refreshProficiencyBonusLabel = function (sheet) {
    let proficiencyBonus = getProficiencyBonus(sheet);
    sheet.get("proficiency").text("+" + proficiencyBonus);
};

const initHPManagement = function (sheet) {
    if(sheet.id() === "main")
        initDeathSaves(sheet);

    initHPIncrementation(sheet);
    initHPContainersToggle(sheet);
};

const initHPIncrementation = function (sheet) {
    const getHPChangeInput = function (sheet) {
        const hpChangeInput = Math.max(1, parseInt(sheet.get("hpInput").value()));
        return hpChangeInput;
    };

    const changeCurrentHP = function (sheet, newHP) {
        const maxValue = getHPMax(sheet);

        if (newHP > maxValue) newHP = maxValue;
        else if (newHP < 0) newHP = 0;

        sheet.get("hp").value(newHP);
    };

    const refreshHPLabels = function () {
        sheet.get('hpLabel').text(getHPCurrent(sheet));
        sheet.get('hpmaxLabel').text(getHPMax(sheet));

        let tempHP = sheet.get('tempHP').value();
        let tempHPLabel = tempHP > 0 ? " +" + tempHP : "";
        sheet.get('tempHPLabel').text(tempHPLabel);
    };

    const refreshDeathSaves = function (){
        if(getHPCurrent(sheet) == 0 && getHPMax(sheet) > 0){
            sheet.get("hpLabel").addClass("text-danger");
            sheet.get('deathSavesEditContainer').show();
            sheet.get('deathSavesDone').show()
            sheet.get('deathSavesReadContainer').hide();
            sheet.get('deathSavesEdit').hide()
        } else {
            resetDeathSaves();
            sheet.get("hpLabel").removeClass("text-danger");
            sheet.get('deathSavesEditContainer').hide();
            sheet.get('deathSavesDone').hide()
            sheet.get('deathSavesReadContainer').show();
            sheet.get('deathSavesEdit').show()
        }
    };

    const resetDeathSaves = function (){
        let blankCheckboxes = {
            deathSaveSuccess1: false,
            deathSaveSuccess2: false,
            deathSaveSuccess3: false,
            deathSaveFailure1: false,
            deathSaveFailure2: false,
            deathSaveFailure3: false
        };
        let foundCheckedBox = false;
        for(checkbox in blankCheckboxes){
            if(sheet.get(checkbox).value())
                foundCheckedBox = true;
        }
        if(foundCheckedBox)
            sheet.setData(blankCheckboxes);
    }

    const refreshHealthDisplays = function() {
        refreshHPLabels();
        if(sheet.id() === "main")
            refreshDeathSaves();
    }

    sheet.get("hpUp").on("click", function () {
        const currentHP = getHPCurrent(sheet);
        const heal = getHPChangeInput(sheet);

        changeCurrentHP(sheet, currentHP + heal);
    })

    sheet.get("hpDown").on("click", function () {
        const currentHP = getHPCurrent(sheet);
        const damage = getHPChangeInput(sheet);

        const tempHP = parseInt(sheet.get("tempHP").value() || 0);
        if (damage <= tempHP) {
            sheet.get("tempHP").value(tempHP - damage);
        } else {
            sheet.get("tempHP").value(0);
            let overflowDamage = damage - tempHP;
            changeCurrentHP(sheet, currentHP - overflowDamage);
        }
    })

    refreshHealthDisplays();
    sheet.get('hp').on("update", refreshHealthDisplays);
    sheet.get('hpmax').on("update", refreshHealthDisplays);
    sheet.get('tempHP').on("update", refreshHealthDisplays);
};

const initHPContainersToggle = function (sheet) {
    const clickToClose = sheet.get("hpEditDoneBtn");
    let currentHP = {
        edit: sheet.get("hp_col"),
        read: sheet.get("hpLabel_col")
    }
    let maxHP = {
        edit: sheet.get("hpmax_col"),
        read: sheet.get("hpmaxLabel_col")
    }
    let tempHP = {
        edit: sheet.get("tempHP_col"),
        read: sheet.get("tempHPLabel_col")
    }

    const closeAllContainers = function () {
        currentHP.edit.hide();
        maxHP.edit.hide();
        tempHP.edit.hide();
        currentHP.read.show();
        maxHP.read.show();
        tempHP.read.show();
        clickToClose.hide();
        sheet.get("addTempHP").show();
        sheet.get("hitPointsContainerEdit").show();
        sheet.get("hitPointsContainerDone").hide();

        if (sheet.id() === "monster" || sheet.id() === "npc")
            sheet.get("hpmax_roll_row").hide();
    }
    const openAllContainers = function () {
        currentHP.edit.show();
        maxHP.edit.show();
        tempHP.edit.show();
        currentHP.read.hide();
        maxHP.read.hide();
        tempHP.read.hide();
        clickToClose.show();
        sheet.get("addTempHP").hide();
        sheet.get("hitPointsContainerEdit").hide();
        sheet.get("hitPointsContainerDone").show();

        if (sheet.id() === "monster" || sheet.id() === "npc")
            sheet.get("hpmax_roll_row").show();
    }
    const refreshClosingButtonVisibility = function () {
        let atLeastOneIsVisible = currentHP.edit.visible() || maxHP.edit.visible() || tempHP.edit.visible()
        if (atLeastOneIsVisible)
            clickToClose.show();
        else clickToClose.hide();
    }
    const toggleContainer = function (container) {
        if (container.edit.visible()) {
            if (container === tempHP) {
                sheet.get("addTempHP").show();
            } else if (container === maxHP && (sheet.id() === "monster" || sheet.id() === "npc")) {
                sheet.get("hpmax_roll_row").hide();
            }
            container.edit.hide();
            container.read.show();
            refreshClosingButtonVisibility();
        } else {
            if (container === tempHP) {
                sheet.get("addTempHP").hide();
            } else if (container === maxHP && (sheet.id() === "monster" || sheet.id() === "npc")) {
                sheet.get("hpmax_roll_row").show();
            }
            container.edit.show();
            container.read.hide();
            refreshClosingButtonVisibility();
        }
    }

    closeAllContainers();

    sheet.get("hpLabel").on("click", function () { toggleContainer(currentHP) });
    sheet.get("hpmaxLabel").on("click", function () { toggleContainer(maxHP) });
    sheet.get('tempHPLabel').on('click', function () { toggleContainer(tempHP) });
    sheet.get("addTempHP").on("click", function () { toggleContainer(tempHP) });
    sheet.get("hitPointsContainerEdit").on("click", openAllContainers);
    sheet.get("hitPointsContainerDone").on("click", closeAllContainers);
    sheet.get("hpEditDoneBtn").on("click", closeAllContainers);
};

const initDeathSaves = function (sheet) {
    initToggleEdit(sheet, "deathSaves");
    sheet.get('deathSaveBtn').on('click', rollDeathSave);
};

const initAttributes = function (sheet) {

    Attributes.each(function (attribute) {
        setAttribute(sheet, attribute.id);

        sheet.get(attribute.id).on("update", function () {
            setAttribute(sheet, attribute.id);
            setSaveLabel(sheet, attribute.id);
            setArmorClassLabel(sheet);
            setInitiativeLabel(sheet);
            setPassiveSkillLabel(sheet, "perception");

            if (sheet.id() === "main") {
                setHitDice(sheet);
                setSkillLabels(sheet);
                setSpellSaveDCLabel(sheet);
                setSpellAttackBonusLabel(sheet);
                setCarryingCapacityLabel(sheet);
            }
        });
        sheet.get(attribute.id + "Modifier").on("click", function () {
            rollAttribute(sheet, attribute.id)
        });
    });
};

const setAttributes = function (sheet) {
    Attributes.each(function (attribute) {
        setAttribute(sheet, attribute.id)
    });
};

const setAttribute = function (sheet, attribute) {
    setAttributeModifierLabel(sheet, attribute);
    if (sheet.id() === "main") {
        displayAttributeScoreBonus(sheet, attribute);
    }
};

const displayAttributeScoreBonus = function (sheet, attribute) {
    let extra = getExtraBonus(sheet, attribute);

    if (extra != 0) {
        sheet.get(attribute).virtualValue(getAttributeScore(sheet, attribute));
    } else {
        sheet.get(attribute).virtualValue(null);
    }
};

const setAttributeModifierLabel = function (sheet, attribute) {
    let modifierValue = getAttributeModifier(sheet, attribute);
    sheet.get(attribute + "Modifier").text(getSign(modifierValue) + modifierValue);
};

const initSaves = function (sheet) {
    Attributes.each(function (attribute) {
        setSaveLabel(sheet, attribute.id);

        if (sheet.id() === "main") {
            setProficiencyIcon(sheet, "save_" + attribute.id);
            sheet.get("save_" + attribute.id + "_prof").on("update", function () {
                setSaveLabel(sheet, attribute.id);
                setProficiencyIcon(sheet, "save_" + attribute.id);
            });

            sheet.get("save_" + attribute.id + "_profIcon").on('click', function() {
                toggleProficiencyCheckbox(sheet, "save_" + attribute.id);
            });

        } else if (sheet.id() === "monster" || sheet.id() === "npc") {
            let tooltipLabel = _("Roll %attributeName Saving throw.").replace("%attributeName", _(attribute.name))
            sheet.get("save_" + attribute.id).setToolTip(tooltipLabel, "bottom");
        }

        sheet.get("save_" + attribute.id).on("click", function () {
            rollSave(sheet, attribute.id)
        });
    })
};

const setSaveLabels = function (sheet) {
    Attributes.each(function (attribute) {
        setSaveLabel(sheet, attribute.id);
    });
};

const setSaveLabel = function (sheet, attribute) {
    let saveBonus = getSavingThrowBonus(sheet, attribute);
    sheet.get("save_" + attribute + "_bonus").text(getSign(saveBonus) + saveBonus);
};

const setProficiencyIcon = function (sheet, field) {

    let hasExpertise = function(field){
        let bonusesRepeater = sheet.get("bonuses").value();
        let expertise = false;
        each(bonusesRepeater, function(entryData, entryId){
            if(entryData.bonusField == field && entryData.expertise)
                expertise = true;
        })
        return expertise;
    }

    let hasHalfProficiency = function(field){
        if(isValidTableElement(Skills, field) && !hasProficiency(field)){
            let hasJackOfAllTrades = sheet.get('jackOfAllTradesCheck').value();
            let hasRemarkableAthlete = sheet.get('remarkableAthleteCheck').value();
            let attribute = Skills.get(field).attribute;
            let isPhysicalSkill = attribute == "strength" || attribute == "dexterity" || attribute == "constitution";
            if(hasRemarkableAthlete && isPhysicalSkill)
                return "remarkableAthlete";
            else if(hasJackOfAllTrades)
                return "jackOfAllTrades";
            else return false;
        } else return false;
    }

    let hasProficiency = function(field){
        return sheet.get(field+"_prof").value();
    }

    let proficiencyType = "";
    if(hasExpertise(field)){
        proficiencyType = "expertise";
        sheet.get(field+'_profIcon').removeClass("opacity-50").setToolTip(_("Expertise"), "left");
    } else if (hasProficiency(field)){
        proficiencyType = "full";
        sheet.get(field+'_profIcon').removeClass("opacity-50").setToolTip(_("Proficient"), "left");
    } else if (hasHalfProficiency(field)){
        proficiencyType = "half";
        sheet.get(field+'_profIcon').addClass("opacity-50")
        if(hasHalfProficiency(field) == "remarkableAthlete")
            sheet.get(field+'_profIcon').setToolTip(_("Half-Proficiency from Remarkable Athlete"), "left");
        else if(hasHalfProficiency(field) == "jackOfAllTrades")
            sheet.get(field+'_profIcon').setToolTip(_("Half-Proficiency from Jack of All Trades"), "left");
    } else {
        proficiencyType = "none";
        sheet.get(field+'_profIcon').addClass("opacity-50").setToolTip("");
    }
    let iconCode = ProficiencyIcons.get(proficiencyType).icon;
    if(sheet.get(field+'_profIcon').value() != iconCode)
        sheet.get(field+'_profIcon').value(iconCode)
};

const setProficiencyIcons_skills = function (sheet) {
    Skills.each(function (skill) {
        setProficiencyIcon(sheet, skill.id);
    });
};

const refreshProficiencyIcons_saves = function (sheet) {
    Attributes.each(function (attribute) {
        setProficiencyIcon(sheet, "save_" + attribute.id);
    });
};

const toggleProficiencyCheckbox = function (sheet, field) {
    if(sheet.get(field+'_prof').value())
        sheet.get(field+'_prof').value(false);
    else sheet.get(field+'_prof').value(true);
};

const initSkills = function (sheet) {
    Skills.each(function (skill) {
        setSkillLabel(sheet, skill.id);
        setProficiencyIcon(sheet, skill.id);

        sheet.get(skill.id + "_profIcon").on('click', function() {
            toggleProficiencyCheckbox(sheet, skill.id);
        });

        sheet.get(skill.id + "_prof").on("update", function () {
            setSkillLabel(sheet, skill.id);
            setProficiencyIcon(sheet, skill.id);
        });
        sheet.get(skill.id).on("click", function () {
            rollSkill(sheet, skill.id)
        });
    });
};

const setSkillLabels = function (sheet) {
    Skills.each(function (skill) {
        setSkillLabel(sheet, skill.id);
    });
};

const setSkillLabel = function (sheet, skill) {
    let skillBonus = getSkillBonus(sheet, skill);
    sheet.get(skill + '_bonus').text(getSign(skillBonus)+skillBonus);

    if(PASSIVE_SKILLS.hasOwnProperty(skill))
        setPassiveSkillLabel(sheet, skill);
};

const initPassiveSkills = function (sheet) {
    for(skill in PASSIVE_SKILLS){
        initPassiveSkill(sheet, skill);
    }
};

const initPassiveSkill = function (sheet, skill) {
    setPassiveSkillLabel(sheet, skill);
    if (sheet.id() === "main" ) {
        let fieldID = PASSIVE_SKILLS[skill].fieldID;
        
        sheet.get(fieldID +'Input').on("update", function () {
            setPassiveSkillLabel(sheet, skill);
        })
        initToggleEdit(sheet, fieldID);
    }
}

const setPassiveSkillLabels = function (sheet) {
    for (skill in PASSIVE_SKILLS){
        setPassiveSkillLabel(sheet, skill);
    }
}

const setPassiveSkillLabel = function (sheet, skill) {
    const passiveSkill = getPassiveSkill(sheet, skill);
    const passiveSkillField = PASSIVE_SKILLS[skill].fieldID;

    sheet.get(passiveSkillField).text(passiveSkill);
}

const initInitiative = function (sheet) {

    setInitiativeLabel(sheet);
    sheet.get("initInput").on("update", function () { setInitiativeLabel(sheet) });
    if (sheet.id() === "main")
        sheet.get("initBonus").on("update", function () { setInitiativeLabel(sheet) });
    sheet.get("initiativeBtn").on("click", function () { rollInitiative(sheet) });
    sheet.get("initiativeBtn").setToolTip(_("Click to roll Initiative!"));

    initToggleEdit(sheet, "initiative");
};

const setInitiativeLabel = function (sheet) {
    let initiative = getInitiative(sheet);
    sheet.get("initiative").text(getSign(initiative) + initiative);
};

const initArmorClass = function (sheet) {

    initToggleEdit(sheet, "ac");
    setArmorClassLabel(sheet);

    sheet.get('acInput').on("update", function () { setArmorClassLabel(sheet) })

    if (sheet.id() === "main") {
        let components = [
            "armorAc",
            "acBonus",
            "acAttributeOne",
            "acAttributeTwo"
        ];

        each(components, function (componentName) {
            sheet.get(componentName).on("update", function () { setArmorClassLabel(sheet) })
        });
    }
};

const setArmorClassLabel = function (sheet) {
    let ac = getArmorClass(sheet);
    sheet.get("ac").text(ac);
};

const initSpeed = function (sheet) {
    setSpeedLabel(sheet);
    if (sheet.id() === "main") {
        sheet.get("speedInput").on("update", function () {
            setSpeedLabel(sheet)
        })
        initToggleEdit(sheet, "speed");
    }
};

const setSpeedLabel = function (sheet) {
    let speed = getSpeed(sheet);
    sheet.get("speed").text(speed);
};

const initQuickResources = function(sheet) {
    
    initToggleEdit(sheet, "quickResource");
    initToggleEdit(sheet, "quickResourceTwo");

    const matchReadEditInputs = function(target){
        let prefix = target.id().split("_")[0];
        let updatedField = target.id();
        let otherField = prefix + (updatedField.includes("_") ? "" : "_read");

        let updatedValue = sheet.get(updatedField).value();
        let otherValue = sheet.get(otherField).value();
        if(otherValue !== updatedValue)
            sheet.get(otherField).value(updatedValue);
    }
    matchReadEditInputs(sheet.get('quickResource'));
    matchReadEditInputs(sheet.get('quickResourceTwo'));
    sheet.get('quickResource_read').on('update', matchReadEditInputs);
    sheet.get('quickResource').on('update', matchReadEditInputs);
    sheet.get('quickResourceTwo_read').on('update', matchReadEditInputs);
    sheet.get('quickResourceTwo').on('update', matchReadEditInputs);
};

const initLongRest = function (sheet) {

    const sortByLargestHitDie = function(a, b){
        let selectedClasses = getSelectedClasses(sheet);
        let hitDieA = selectedClasses[a].hitDie;
        let hitDieB = selectedClasses[b].hitDie;
        let hitDieSizeA = parseInt(hitDieA.replace("d",""));
        let hitDieSizeB = parseInt(hitDieB.replace("d",""));
        return hitDieSizeB - hitDieSizeA;
    };

    const getRecoverySelection = function() {
        let selectedClasses = getSelectedClasses(sheet);
        let recoverySelection = {};
        let nbRecoverableHitDice = Math.floor(getLevel(sheet) / 2);

        let classesArray = Object.keys(selectedClasses);
        classesArray.sort(sortByLargestHitDie);
        classesArray.forEach(function(classChoice, index, arr){
            let thisClass = selectedClasses[classChoice];
            let missingHitDice = thisClass.level - thisClass.hitdiceCurrent;
            let recoveredHitDice = Math.min(missingHitDice, nbRecoverableHitDice);
            
            if(recoveredHitDice > 0) {
                recoverySelection[classChoice] = recoveredHitDice;
                nbRecoverableHitDice -= recoveredHitDice;
            }                
        })
        return recoverySelection;
    };

    const setRecoveryRecap = function (promptView){
        let selectedClasses = getSelectedClasses(sheet);
        let nbRecoverableHitDice = 0;       
        let recoveryDetail = "";

        let recoverySelection = getRecoverySelection();       
        for(classChoice in recoverySelection){
            let thisClass = selectedClasses[classChoice];
            let recoveredHitDice = recoverySelection[classChoice];
            let classDisplayName = (isValidClass(sheet, thisClass.id) && thisClass.id != "null") ? thisClass.name : "[Class not selected]"

            if(recoveryDetail)
                recoveryDetail+= " · ";

            recoveryDetail += classDisplayName + " " + recoveredHitDice + thisClass.hitDie;
            nbRecoverableHitDice += recoveredHitDice;
        }

        let recoveryHeader = _("Will recover %nbHitDice Hit Dice.").replace("%nbHitDice", nbRecoverableHitDice);
        if(nbRecoverableHitDice == 0) recoveryDetail = _("All Hit Dice at maximum.")
        promptView.get("recoverHalfHitDice_headerLabel").text(recoveryHeader);
        promptView.get("recoverHalfHitDice_detailLabel").text(recoveryDetail);
    };
    
    const regainHalfOfHitDice = function (dataToSet){
        let recoverySelection = getRecoverySelection();
        let selectedClasses = getSelectedClasses(sheet);
        let hitDiceMore_new = sheet.get('hitDiceMore').value() || {};

        for(classIndex in recoverySelection){
            let thisClass = selectedClasses[classIndex];
            let newHitDiceCurrent = thisClass.hitdiceCurrent + recoverySelection[classIndex];

            if(classIndex == "baseClass" || classIndex == "firstMulticlass")
                dataToSet[thisClass.currentHitDiceField] = newHitDiceCurrent;
            else {
                hitDiceMore_new[classIndex]["hitdiceCurrent"] = newHitDiceCurrent;
            }
        }
        dataToSet["hitDiceMore"] = hitDiceMore_new;

        return dataToSet;
    };


    const restoreSpellSlots = function (dataToSet) {
        const getSpellSlots = function (level) {
            let spellSlots = sheet.getData()["spellSlot" + level]

            if (spellSlots) {
                return parseInt(spellSlots);
            }

            return 0;
        };

        dataToSet["spellLeft1"] = getSpellSlots("1");
        dataToSet["spellLeft2"] = getSpellSlots("2");
        dataToSet["spellLeft3"] = getSpellSlots("3");
        dataToSet["spellLeft4"] = getSpellSlots("4");
        dataToSet["spellLeft5"] = getSpellSlots("5");
        dataToSet["spellLeft6"] = getSpellSlots("6");
        dataToSet["spellLeft7"] = getSpellSlots("7");
        dataToSet["spellLeft8"] = getSpellSlots("8");
        dataToSet["spellLeft9"] = getSpellSlots("9");

        let selectedLevel = sheet.get("spellFilter").value();
        if (selectedLevel != "all") {
            dataToSet["spellLeft"] = dataToSet["spellLeft" + selectedLevel];
        }

        return dataToSet;
    }

    const applyLongRest = function(viewData) {
        let dataToSet = {};

        if(viewData.recoverHP){
            dataToSet['hp'] = getHPMax(sheet);
            dataToSet['hpInput'] = 0;
            if(viewData.clearTempHP)
                dataToSet['tempHP'] = 0;
        }

        if(viewData.regainHalfHitDice)
            dataToSet = regainHalfOfHitDice(dataToSet);

        if(viewData.restoreSpellSlots)
            dataToSet = restoreSpellSlots(dataToSet);

        sheet.setData(dataToSet);
        rechargeResources(sheet, viewData);
    };

    const initPromptView = function(promptView){
        promptView.get('recoverHP').value(true);
        promptView.get('clearTempHP').value(true);
        promptView.get('regainHalfHitDice').value(true);
        promptView.get('restoreSpellSlots').value(true);
        setRecoveryRecap(promptView);

        initResourcesInPromptView(sheet, promptView, "longRest");
    };

    sheet.get('longrest').on('click', function () {
        sheet.prompt(_('Take a Long Rest'), 'promptLongRest', applyLongRest, initPromptView);
    });
};

const initShortRest = function (sheet) {
    sheet.get("shortrest").on("click", function () {
        sheet.prompt(_("Take a Short Rest"), "promptShortRest", 
            function (viewData) {
                rechargeResources(sheet, viewData);
            },
            function (promptView) {
                initHitDiceInPromptView(sheet, promptView);
                initResourcesInPromptView(sheet, promptView, "shortRest");
            }
        );
    })
};

const rechargeResources = function(sheet, viewData){
    let dataToSet = {};
    for(index in viewData.resources){
        if(viewData.resources[index].quickResourceRecharge){
            if(index == "quickResource") {
                if(sheet.get('quickResource').value() != sheet.get('quickResourceMax').value())
                    dataToSet["quickResource"] = sheet.get('quickResourceMax').value();
            } else if(index == "quickResourceTwo") {
                if(sheet.get('quickResourceTwo').value() != sheet.get('quickResourceMaxTwo').value())
                    dataToSet["quickResourceTwo"] = sheet.get('quickResourceMaxTwo').value();
            } else {
                if(!dataToSet.hasOwnProperty("resources"))
                    dataToSet["resources"] = sheet.get("resources").value();
                dataToSet["resources"][index]["quickResource"] = sheet.get("resources").value()[index]["quickResourceMax"];
            }
        }
    };
    sheet.setData(dataToSet);
};
    
const initResourcesInPromptView = function(sheet, promptView, restType){
    let resources = {}

    let hasQuickResource = sheet.get("quickResourceName").value() && sheet.get("quickResourceMax").value();
    let resourceRechargesOnThisRest = restType == "shortRest" ? sheet.get("rechargesOnShortRest").value() : sheet.get("rechargesOnLongRest").value();
    if(hasQuickResource && resourceRechargesOnThisRest){
        resources["quickResource"]= {
            "quickResourceRecharge": true,
            "quickResourceName": sheet.get('quickResourceName').value(),
            "quickResource": sheet.get('quickResource').value(),
            "quickResourceMax": sheet.get('quickResourceMax').value()
        };
    };

    let hasQuickResourceTwo = sheet.get("quickResourceNameTwo").value() && sheet.get("quickResourceMaxTwo").value();
    let resourceTwoRechargesOnThisRest = restType == "shortRest" ? sheet.get("rechargesOnShortRestTwo").value() : sheet.get("rechargesOnLongRestTwo").value();
    if(hasQuickResourceTwo && resourceTwoRechargesOnThisRest){
        resources["quickResourceTwo"]= {
            "quickResourceRecharge": true,
            "quickResourceName": sheet.get('quickResourceNameTwo').value(),
            "quickResource": sheet.get('quickResourceTwo').value(),
            "quickResourceMax": sheet.get('quickResourceMaxTwo').value()
        };
    }

    if(sheet.get('resources').value()){
        each(sheet.get('resources').value(), function(entryData, entryId){
            let rechargesOnToggle = restType == "shortRest" ? "rechargesOnShortRest" : "rechargesOnLongRest";
            if(entryData[rechargesOnToggle]){
                resources[entryId] = entryData;
                resources[entryId]["quickResourceRecharge"] = true;
            }
        })
    }

    if(Object.keys(resources).length > 0)
        promptView.get('resources').value(resources);
    else {
        promptView.get("resourcesRow").hide();
        promptView.get("noApplicableResources_error").show();
    }
}

const initHitDiceInPromptView = function(sheet, promptView){
    let hitDiceRepeater = promptView.get("hitDiceMore");

    const setHitDieInSheet = function (target){
        let selectedClasses = getSelectedClasses(sheet);
        let index = target.index();
        let thisClass = selectedClasses[index];
        let newHitDiceCurrent = target.value() || 0;
        let currentHitDiceComponent;

        if(index == "baseClass" || index == "firstMulticlass")
            currentHitDiceComponent = sheet.get(thisClass.currentHitDiceField);
        else
            currentHitDiceComponent = sheet.get("hitDiceMore").find(index).find(thisClass.currentHitDiceField);
        
        if(currentHitDiceComponent.value() != newHitDiceCurrent)
            currentHitDiceComponent.value(newHitDiceCurrent);
    };

    const limitCurrentHitDiceInPromptView = function (target) {
        let currentHitDice = promptView.get("hitDiceMore").find(target.index()).find("hitdiceCurrent").value();
        let maxHitDice = promptView.get("hitDiceMore").find(target.index()).find("hitdiceMax").value();

        if (currentHitDice > maxHitDice)
            target.value(maxHitDice);
        else if (currentHitDice < 0)
            target.value(0);
    };

    const setButtonState = function (target) {
        let entryData = promptView.get("hitDiceMore").find(target.index()).value();
        let entryBtn = promptView.get("hitDiceMore").find(target.index()).find('hitdiceBtn');
        let currentHitDice = entryData.hitdiceCurrent || 0;
        let currentDieType = entryData.hitdiceType || ":question:";
        if (currentHitDice == 0)
            entryBtn.addClass("disabled").setToolTip(_("No remaining Hit Dice. Take a Long Rest to replenish half your Hit Dice."), "right")
        else if (currentDieType == ":question:")
            entryBtn.addClass("disabled").setToolTip(_("No class selected, cannot retrieve hit die type."), "right")
        else
            entryBtn.removeClass("disabled").setToolTip(_("Roll Hit Dice to Recover HP!"), "right");
    };

    const rollIfNotDisabled = function (target) {
        let entryBtn = promptView.get("hitDiceMore").find(target.index()).find('hitdiceBtn');
        if(!entryBtn.hasClass('disabled'))
            rollHitDice(target, sheet);
    };
    setHitDice(sheet);
    setHitDiceInPromptView(sheet, promptView);

    hitDiceRepeater.on('click', "hitdiceBtn", rollIfNotDisabled);
    
    each(hitDiceRepeater.value(), function(entryData, entryId){
        let hitdiceCurrentField = hitDiceRepeater.find(entryId).find("hitdiceCurrent");

        hitdiceCurrentField.on('update', function(target){
            setHitDieInSheet(target);
            setButtonState(target);
            limitCurrentHitDiceInPromptView(target);          
        });
        setButtonState(hitdiceCurrentField);
        limitCurrentHitDiceInPromptView(hitdiceCurrentField);
    })
};

const setHitDiceInPromptView = function (sheet, promptView) {
    let hitDice = {};
    let selectedClasses = getSelectedClasses(sheet);

    for (classIndex in selectedClasses){
        let thisClass = selectedClasses[classIndex];
        let classDisplayName = _("[Class not selected]")
        let classDisplayHitDie = ":question:";
        if(isValidClass(sheet, thisClass.id) && thisClass.id != "null"){
            classDisplayName = thisClass.name;
            if(thisClass.hitDie)
                classDisplayHitDie = "1"+_(thisClass.hitDie)+"+"+getAttributeModifier(sheet,"constitution");
        }
        hitDice[classIndex] = {
            "hitdiceClass": _(classDisplayName),
            "hitdiceCurrent": thisClass.hitdiceCurrent,
            "hitdiceMax": thisClass.level,
            "hitdiceType": classDisplayHitDie
        };
    }
    promptView.get("hitDiceMore").value(hitDice);
};

const initWeapons = function (sheet) {
    const weapons = sheet.get('weapons');

    const setBindings = function () {
        Bindings.clear("weapons");

        each(weapons.value(), function (weaponData) {
            let attackStName = "n/a";
            let attackStDC = "";
            if (isValidTableElement(Attributes, weaponData.attackSt)) {
                attackStName = Attributes.get(weaponData.attackSt).name || "n/a";
                attackStDC = weaponData.attackStDC || "??";
            }

            Bindings.add(weaponData.weaponName, "weapons", "bindingWeapon", function () {
                return {
                    weaponName: weaponData.weaponName || _("[Unnamed Weapon]"),
                    weaponActionType: weaponData.weaponActionType,
                    weaponRange: weaponData.weaponRange,
                    weaponDamages: weaponData.weaponDamages,
                    weaponType: weaponData.weaponType,
                    attackSt: attackStName,
                    attackStDC: attackStDC,
                    weaponDescription: weaponData.weaponDescription
                };
            });
        });
    };

    const sendDetailsToChat = function (target) {
        let weaponData = weapons.value()[target.index()];
        if (weaponData.weaponName)
            Bindings.send(sheet, weaponData.weaponName);
    };

    const refreshWeaponsDisplay = function () {
        toggleHeaderRow(sheet, weapons);
        each(weapons.value(), function (entryData, entryId) {
            let entry = weapons.find(entryId)

            if (entryData.ammoLeft)
                entry.find("ammoRow").show();
            else entry.find("ammoRow").hide();

            if (isValidTableElement(Attributes, entryData.attackSt)) {
                let attackSavingThrowLabel;
                attackSavingThrowLabel = _(Attributes.get(entryData.attackSt).name);
                entry.find("attackSavingThrowLabel").text(attackSavingThrowLabel);
            } else entry.find("attackSavingThrowRow").hide();
        })
    };

    const setWeapons = function () {
        setBindings();
        refreshWeaponsDisplay();
    }

    const rollWeaponAttack = function (weapon) {
        let weaponName = weapon.find("weaponName").value();

        let weaponAttribute = weapon.find("weaponAttribute").value();
        let attributeModifier = getAttributeModifier(sheet, weaponAttribute);
        let hasProficiency = weapon.find("weaponProficient").value();
        let weaponBonus = parseInt(weapon.find("weaponBonus").value());
        let weaponAttackBonus = attributeModifier + (hasProficiency ? getProficiencyBonus(sheet) : 0) + weaponBonus;

        let weaponDamages = weapon.find('weaponDamages').value() || 0;
        if (weaponDamages)
            weaponDamages += "+" + getAttributeModifier(sheet, weaponAttribute) + "+" + weaponBonus;

        rollAttack(sheet, weaponName, weaponAttackBonus, weaponDamages);

        if (sheet.get('alwaysSendToChat').value())
            sendDetailsToChat(weapon);
    }

    const incrementAmmo = function (weapon) {
        let remainingAmmo = parseInt(weapon.find('ammoLeft').value());

        if (remainingAmmo > 0) {
            remainingAmmo--;
            weapon.find("ammoLeft").value(remainingAmmo);
        }
    }


    setWeapons();
    weapons.on("update", setWeapons);
    weapons.on('click', 'buttonSendToChat', sendDetailsToChat)
    weapons.on('click', 'nameLabel', function (target) {
        let weapon = weapons.find(target.index());
        rollWeaponAttack(weapon);
        incrementAmmo(weapon);
    });
    each(weapons.value(), function (entryData, entryId) {
    });
};

const initSpells = function (sheet) {

    const spells = sheet.get("spells");

    const setSpellSlots = function () {
        let dataToSet = {};
        let selectedLevel = sheet.get("spellFilter").value();

        let currentSpellSlots = sheet.get("spellSlot").value();
        let currentSpellsLeft = sheet.get("spellLeft").value();

        if (currentSpellSlots < 0) {
            currentSpellSlots = 0;
            dataToSet["spellSlot"] = currentSpellSlots;
        }
        if (currentSpellsLeft < 0) {
            currentSpellsLeft = 0;
            dataToSet["spellLeft"] = currentSpellsLeft;
        } else if (currentSpellsLeft > currentSpellSlots) {
            currentSpellsLeft = currentSpellSlots
            dataToSet["spellLeft"] = currentSpellsLeft;
        };

        dataToSet["spellSlot" + selectedLevel] = currentSpellSlots;
        dataToSet["spellLeft" + selectedLevel] = currentSpellsLeft;

        sheet.setData(dataToSet);
    };

    const filterSpells = function () {
        let selectedLevel = sheet.get("spellFilter").value();
        let sheetData = sheet.getData();

        if (selectedLevel == "all" || selectedLevel == "null" || selectedLevel == "1") {
            sheet.get("slotsContainer").hide();
        } else {
            sheet.get("slotsContainer").show();
            let spellSlots = sheetData["spellSlot" + selectedLevel] || 0;
            let spellsLeft = sheetData["spellLeft" + selectedLevel] || 0;

            if (sheet.get("spellSlot").value() !== spellSlots)
                sheet.get("spellSlot").value(spellSlots);
            if (sheet.get("spellLeft").value() !== spellsLeft)
                sheet.get("spellLeft").value(spellsLeft);
        }

        each(sheet.get("spells").value(), function (spellData, spellId) {
            if (spellData.spellLevel == selectedLevel || selectedLevel == "all") {
                sheet.get("spells").find(spellId).show();
            } else {
                sheet.get("spells").find(spellId).hide();
            }
        });
    };

    const setBindings = function () {
        Bindings.clear("spells");

        each(spells.value(), function (spellData, spellId) {
            Bindings.add(spellData.spellName, "spells", "bindingSpell", function () {
                let binding = {
                    spellName: spellData.spellName || _("[Unnamed Spell]"),
                    spellLevel: 0,
                    spellSchool: 0,
                    ritualCast: spellData.ritualCast,
                    spellTime: spellData.spellTime,
                    spellDuration: spellData.spellDuration,
                    concentration: spellData.concentration,
                    spellRange: spellData.spellRange,
                    spellComponentV: spellData.spellComponentV,
                    spellComponentS: spellData.spellComponentS,
                    spellComponentM: spellData.spellComponentM,
                    spellComponentDescription: spellData.spellComponentDescription,
                    spellAttack: 0,
                    spellSaveDC: 0,
                    spellDamage: spellData.spellDamage,
                    spellDescription: spellData.spellDescription
                };
                if (isValidTableElement(SpellLevels, spellData.spellLevel.toString()))
                    binding["spellLevel"] = _(SpellLevels.get(spellData.spellLevel.toString()).name)
                if (isValidTableElement(SpellSchools, spellData.spellSchool))
                    binding["spellSchool"] = _(SpellSchools.get(spellData.spellSchool).name)
                if (isValidTableElement(SpellAttacks, spellData.spellAttack)) {
                    binding["spellAttack"] = _(SpellAttacks.get(spellData.spellAttack).name)
                    if (spellData.spellAttack != "ranged" && spellData.spellAttack != "melee")
                        binding["spellSaveDC"] = getSpellSaveDC(sheet);
                }
                return binding;
            });
        });
    };

    const refreshPreparedSpellOpacity = function (index){
        let entry = spells.find(index);
        let entryData = entry.value();
        if(entryData.spellPrepared || entryData.alwaysPrepared || entryData.spellLevel === "1")
            entry.find('repeaterSpell_read').removeClass("opacity-25");
        else entry.find('repeaterSpell_read').addClass("opacity-25");
    };

    const getCurrentlyPreparedSpellsCount = function (sheet) {
        let counter = 0;
        each(spells.value(), function(spellData, spellId){
            if (spellData.spellPrepared && !spellData.alwaysPrepared && spellData.spellLevel !== "1")
                counter++;
        })
        return counter;
    };

    const setPreparedSpellsCounter = function () {
        let counter = getCurrentlyPreparedSpellsCount(sheet);
        sheet.get("currentPreparedSpells").text(counter);
    };

    const sendSpellDetailsToChat = function (target) {
        let spellData = spells.value()[target.index()];
        if (spellData.spellName)
            Bindings.send(sheet, spellData.spellName);
    };

    const refreshSpellReadView_all = function () {
        each(spells.value(), function(spellData, spellId){
            refreshSpellReadView_one(spellId);
        });
    };

    const refreshSpellReadView_one = function (index) {
        let spell = spells.find(index);
        let spellData = spell.value();

        let spellLevelLabel;
        if (spellData.spellLevel && spellData.spellLevel != "0") {
            spellLevelLabel = _(SpellLevels.get(spellData.spellLevel.toString()).name);
            spell.find("spellLevelLabel").text(spellLevelLabel);
        } else {
            spell.find("spellLevelLabel").hide();
        }

        let spellSchoolLabel;
        if (spellData.spellSchool && spellData.spellSchool != "0") {
            spellSchoolLabel = _(SpellSchools.get(spellData.spellSchool).name);
            spell.find("spellSchoolLabel").text(spellSchoolLabel);
        } else {
            spell.find("spellSchoolLabel").hide();
        }

        let spellAttackLabel = _("n/a");
        if (isValidTableElement(SpellAttacks, spellData.spellAttack)) {
            spellAttackLabel = _(SpellAttacks.get(spellData.spellAttack).name);
        }

        if (spellData.concentration)
            spell.find("concentrationDisplay").show();
        if (spellData.ritualCast)
            spell.find("ritualCastDisplay").show();
        if (spellData.materialCost)
            spell.find("materialCostDisplay").show();
        if(spellData.alwaysPrepared || spellData.spellLevel === "1"){
            spell.find("spellPrepared").hide();
            spell.find("alwaysPrepared_read").show();
        } else {
            spell.find("spellPrepared").show();
            spell.find("alwaysPrepared_read").hide();
        };
        if (spellData.spellPrepared || spellData.alwaysPrepared || spellData.spellLevel === "1")
            spell.find('repeaterSpell_read').removeClass('opacity-25');
        else spell.find('repeaterSpell_read').addClass('opacity-25');

        spell.find("spellAttackLabel").text(spellAttackLabel);
    };

    const rollSpellAttack = function (target) {
        let spellData = spells.value()[target.index()];
        let selectedLevel = sheet.get("spellFilter").value();
        let spellSlotsLeft = sheet.get("spellLeft").value();

        if (selectedLevel === "1" || selectedLevel === "all" || spellSlotsLeft > 0) {
            if (sheet.get('alwaysSendToChat').value())
                sendSpellDetailsToChat(target);

            if (spellSlotsLeft > 0)
                sheet.get("spellLeft").value(spellSlotsLeft - 1);

            if (spellData.spellAttack === "ranged" || spellData.spellAttack === "melee") {
                rollAttack(sheet, spellData.spellName, getSpellAttackBonus(sheet), spellData.spellDamage);

            } else if (spellData.spellDamage) {
                rollDamages(sheet, spellData.spellName, spellData.spellDamage, false)
            };
        }
        else errorPrompt(sheet, "noSpellSlotsRemaining");
    };

    const spellsInitCallback = function (repeaterComponent, index){
        toggleHeaderRow(sheet, spells);
        filterSpells();
        refreshSpellReadView_one(index);
        setPreparedSpellsCounter();
        setBindings();

        repeaterComponent.find(index).find("spellPrepared").on('update', function(target){
            refreshPreparedSpellOpacity(target.index());
            setPreparedSpellsCounter();
        })
    };

    const spellsUpdateCallback = function (repeaterComponent, index) {
        let repeaterValue = repeaterComponent.value() || {};
        if (repeaterValue.hasOwnProperty(index))
            spellsInitCallback(repeaterComponent, index);
    };

    initToggleEdit(sheet, "spellSaveDC");
    setSpellSaveDCLabel(sheet);
    sheet.get("spellSaveDcInput").on("update", function () {
        setSpellSaveDCLabel(sheet);
    });

    initToggleEdit(sheet, "spellAttackBonus");
    setSpellAttackBonusLabel(sheet);
    sheet.get("spellAttackBonusInput").on("update", function () {
        setSpellAttackBonusLabel(sheet);
    });

    sheet.get("spellcastingAbility").on("update", function () {
        setSpellSaveDCLabel(sheet);
        setSpellAttackBonusLabel(sheet);
    });

    setSpellSlots();
    sheet.get("spellSlot").on("update", setSpellSlots);
    sheet.get("spellLeft").on("update", setSpellSlots);

    filterSpells();
    sheet.get("spellFilter").on("update", filterSpells);

    toggleHeaderRow(sheet, spells);
    refreshSpellReadView_all();
    setPreparedSpellsCounter();
    setBindings();

    initRepeater(spells, spellsInitCallback, spellsUpdateCallback)
    spells.on('click', 'buttonSendToChat', sendSpellDetailsToChat);
    spells.on('click', 'nameLabel', rollSpellAttack);
};

const setSpellSaveDCLabel = function (sheet) {
    const spellSaveDC = getSpellSaveDC(sheet);
    sheet.get("spellSaveDc").text(spellSaveDC);
};

const setSpellAttackBonusLabel = function (sheet) {
    const spellAttackBonus = getSpellAttackBonus(sheet);
    sheet.get("spellAttackBonus").text(getSign(spellAttackBonus)+spellAttackBonus);
};

const initItems = function (sheet) {
    const items = sheet.get('items');

    const setBindings = function () {
        Bindings.clear("items");

        each(items.value(), function (itemData, itemId) {
            let objectTypeText = "n/a";
            let hasObjectType = itemData.objectType && Array.from(itemData.objectType).length > 1
            if (hasObjectType)
                objectTypeText = _(ObjectTypes.get(itemData.objectType).name);

            Bindings.add(itemData.itemName, "items", "bindingItem", function () {
                let binding = {
                    itemName: itemData.itemName || _("[Unnamed Item]"),
                    magical: itemData.magical,
                    objectType: objectTypeText,
                    actionType: itemData.actionType,
                    effectDuration: itemData.effectDuration,
                    rechargesShortRest: itemData.rechargesShortRest,
                    rechargesLongRest: itemData.rechargesLongRest,
                    rechargesOther: itemData.rechargesOther,
                    requiresAttunement: itemData.requiresAttunement,
                    itemDescription: itemData.itemDescription
                };
                return binding;
            });
        });
    };

    const sendDetailsToChat = function (target) {
        let itemData = items.value()[target.index()];
        if (itemData.itemName)
            Bindings.send(sheet, itemData.itemName);
    };

    const refreshItemLabels = function () {

        each(items.value(), function (itemData, itemId) {
            let item = items.find(itemId);

            if (itemData.requiresAttunement) {
                if (itemData.attuned) {
                    item.find('labelAttuned').removeClass('d-none').addClass('d-inline');
                    item.find('labelUnattuned').addClass('d-none').removeClass('d-inline');
                } else {
                    item.find('labelUnattuned').removeClass('d-none').addClass('d-inline');
                    item.find('labelAttuned').addClass('d-none').removeClass('d-inline');
                }
            } else {
                item.find('labelUnattuned').addClass('d-none').removeClass('d-inline');
                item.find('labelAttuned').addClass('d-none').removeClass('d-inline');
            }
            let hasObjectType = itemData.objectType && Array.from(itemData.objectType).length > 1
            let shouldDisplayFirstCol = hasObjectType || itemData.actionType || itemData.rechargesShortRest || itemData.rechargesLongRest || itemData.rechargesOther || itemData.effectDuration
            let shouldDisplaySecondCol = itemData.unitValue || itemData.unitWeight;
            if (shouldDisplayFirstCol) {
                item.find('item_firstCol').show();

                if (hasObjectType) {
                    item.find('objectType_row').show();
                    let objectType = _(ObjectTypes.get(itemData.objectType).name);
                    item.find('objectType_read').text(objectType);
                }
                else item.find('objectType_row').hide();

                if (itemData.actionType)
                    item.find('actionType_row').show()
                else item.find('actionType_row').hide();

                if (itemData.rechargesShortRest || itemData.rechargesLongRest || itemData.rechargesOther)
                    item.find('rechargesAfter_row').show()
                else item.find('rechargesAfter_row').hide();

                if (itemData.effectDuration)
                    item.find('effectDuration_row').show()
                else item.find('effectDuration_row').hide();

            } else item.find('item_firstCol').hide();

            if (shouldDisplaySecondCol) {
                item.find('item_secondCol').show();

                if (itemData.unitValue) {
                    item.find('unitValue_row').show()
                    let currencyLabel = _(Currencies.get(itemData.valueCurrency || "gold").nameShort);
                    item.find('itemCurrency').text(currencyLabel);
                }
                else item.find('unitValue_row').hide();

                if (itemData.unitWeight)
                    item.find('unitWeight_row').show()
                else item.find('unitWeight_row').hide();
            } else item.find('item_secondCol').hide();

            refreshItemWeightAndValueLabels(itemId);
        });
    };

    const refreshAttunedCount = function () {
        let currentCount = 0;
        each(items.value(), function (itemData, itemId) {
            if (itemData.requiresAttunement && itemData.attuned)
                currentCount++;
        })
        sheet.get('currentlyAttuned').text(currentCount);
        if (currentCount > 3)
            errorPrompt(sheet, "exceedMaxAttuned");
    }

    const refreshWeightTotal = function () {
        let currentCount = 0;
        each(items.value(), function (itemData, itemId) {
            if (itemData.unitWeight)
                currentCount += parseInt(itemData.units || 1) * parseInt(itemData.unitWeight);
        })
        sheet.get('currentlyCarried').text(currentCount);
    }

    const refreshItemValueTotal = function () {
        let currentCount = 0;
        each(items.value(), function (itemData, itemId) {
            if (itemData.unitValue) {
                let conversionRate = Currencies.get(itemData.valueCurrency || "gold").conversationRateToGP;
                currentCount += parseInt(itemData.units || 1) * itemData.unitValue * conversionRate;
            }
        })
        let itemsTotalValueLabel = currentCount.toLocaleString() + " " + _(Currencies.get("gold").nameShort);
        sheet.get('itemsTotalValueLabel').text(itemsTotalValueLabel);
    }

    const refreshItemWeightAndValueLabels = function (entryId) {
        let entry = items.find(entryId);
        let itemData = items.value()[entryId];

        let units = itemData.units || 0;
        let unitWeight = itemData.unitWeight || 0;
        let unitValue = itemData.unitValue || 0;
        let valueCurrency = _(Currencies.get(itemData.valueCurrency || "gold").nameShort);

        entry.find('weightTotal').text(unitWeight * units || "-")
        entry.find('valueTotal').text(unitValue * units == 0 ? "-" : (unitValue * units).toLocaleString() + valueCurrency)
    }

    const setItems = function () {
        toggleHeaderRow(sheet, items);
        refreshAttunedCount();
        refreshWeightTotal();
        refreshItemValueTotal();

        each(items.value(), function (entryData, entryId) { //add update behavior on units numberInput at launch and after every update
            items.find(entryId).find('units').on('update', function () {
                refreshItemWeightAndValueLabels(entryId);
                refreshWeightTotal();
                refreshItemValueTotal();
            })
        })
        refreshItemLabels();
        setBindings();
    }

    setItems();
    items.on("update", setItems);
    items.on('click', 'buttonSendToChat', sendDetailsToChat);
    initEncumbrance(sheet);
}

const initEncumbrance = function (sheet) {

    const toggleEncumbranceRow = function (target) {
        const shouldOpen = target.id().includes("Open");
        if (shouldOpen) {
            sheet.get('carryingCapacityInput').show();
            sheet.get('carryingCapacityLabel').hide();
            sheet.get('carryingCapacityOpen').hide();
            sheet.get('carryingCapacityClose').show();
        } else {
            sheet.get('carryingCapacityInput').hide();
            sheet.get('carryingCapacityLabel').show();
            sheet.get('carryingCapacityOpen').show();
            sheet.get('carryingCapacityClose').hide();
        }
    }

    sheet.get('carryingCapacityOpen').on('click', toggleEncumbranceRow);
    sheet.get('carryingCapacityClose').on('click', toggleEncumbranceRow);

    setCarryingCapacityLabel(sheet);
    sheet.get('carryingCapacityInput').on('update', function () {
        setCarryingCapacityLabel(sheet);
    })
}

const setCarryingCapacityLabel = function (sheet) {
    if (!sheet.get('hideEncumbrance').value()) {
        let weightUnit = sheet.get('unitWeight').value() || "lb";
        let weightUnitLabel = _(WeightUnits.get(weightUnit).nameShort);
        let maxCapacity = 0;

        if (sheet.get('carryingCapacityInput').value()) {
            maxCapacity = sheet.get('carryingCapacityInput').value()
        } else {
            let strengthMultiplier = WeightUnits.get(weightUnit).capacityMultiplier
            maxCapacity = getAttributeScore(sheet, "strength") * strengthMultiplier;
        }
        sheet.get('carryingCapacityLabel').text(maxCapacity);
        sheet.get('carryingCapacityUnitLabel').text(weightUnitLabel);
    }
}

const initFeatures = function (sheet) {
    const features = sheet.get("features");

    const setBindings = function () {
        Bindings.clear("features");

        each(features.value(), function (entryData, entryId) {
            Bindings.add(entryData.featureName, "features", "bindingFeature", function () {
                return {
                    featureName: entryData.featureName || _("[Unnamed Feature]"),
                    featureSource: entryData.featureSource,
                    actionType: entryData.actionType,
                    effectDuration: entryData.effectDuration,
                    uses: entryData.uses,
                    rechargesShortRest: entryData.rechargesShortRest,
                    rechargesLongRest: entryData.rechargesLongRest,
                    rechargesOther: entryData.rechargesOther,
                    featureDescription: entryData.featureDescription
                };
            });
        });
    };

    const sendDetailsToChat = function (target) {
        let entryData = features.value()[target.index()]
        if (entryData.featureName)
            Bindings.send(sheet, entryData.featureName);
    };

    const setFeatureLabels = function () {

        each(features.value(), function (featureData, featureId) {
            let feature = features.find(featureId);

            if (featureData.featureSource)
                feature.find('featureSourceLabel').show()
            else feature.find('featureSourceLabel').hide();

            if (featureData.actionType)
                feature.find('actionType_row').show()
            else feature.find('actionType_row').hide();

            if (featureData.uses)
                feature.find('uses_row').show()
            else feature.find('uses_row').hide();

            if (featureData.rechargesShortRest || featureData.rechargesLongRest || featureData.rechargesOther)
                feature.find('rechargesAfter_row').show()
            else feature.find('rechargesAfter_row').hide();

            if (featureData.effectDuration)
                feature.find('effectDuration_row').show()
            else feature.find('effectDuration_row').hide();
        });
    };

    const setFeatures = function () {
        setBindings();
        toggleHeaderRow(sheet, features);
        setFeatureLabels();
    }

    setFeatures();
    features.on("update", setFeatures);
    features.on('click', 'buttonSendToChat', sendDetailsToChat)
};

const initProperties = function (sheet) {
    let propertiesRepeaterId = PROPERTIES_IDS[sheet.id()].repeaterId;
    let properties = sheet.get(propertiesRepeaterId);

    const displayProperties = function () {
        each(properties.value(), function (entryData, entryId) {
            let property = properties.find(entryId);
            let propertyName = getPropertyName(sheet, entryId);
            property.find("propertyDisplay").text(propertyName);
        });
    };

    const setProperties = function () {
        displayProperties();
        setSpeedLabel(sheet);
        if (sheet.id() === "monster" || sheet.id() === "npc") {
            setSaveLabels(sheet);
            setPassiveSkillLabel(sheet, "perception");
            setMonsterGeneralTab(sheet);
            toggleLegendaryResistancesRow(sheet);
        }
    };

    displayProperties();
    properties.on("update", setProperties);
};

const initBonusAndExpertise = function (sheet) {
    const bonuses = sheet.get("bonuses");

    const displayBonusNames = function () {
        each(bonuses.value(), function (entryData, entryId) {
            let bonusType = entryData.bonusField || "null";
            let bonusName = _(Bonuses.get(bonusType).name);
            let bonusDisplayLabel = bonuses.find(entryId).find("bonusDisplay");

            bonusDisplayLabel.text(bonusName);
        });
    };

    displayBonusNames(sheet);
    bonuses.on("update", function () {
        displayBonusNames();
        setAttributes(sheet);
        setSaveLabels(sheet);
        setSkillLabels(sheet);
        setProficiencyIcons_skills(sheet);
        refreshProficiencyIcons_saves(sheet);
    });
};

const initSettings = function (sheet) {

    const setWeightUnit = function () {
        setCarryingCapacityLabel(sheet);
        let weightUnit = _(WeightUnits.get(sheet.get('unitWeight').value() || "lb").nameShort);

        //tabItems Header Tooltip
        let tooltipLabel = _("In %unit").replace("%unit", weightUnit)
        sheet.get('weightHeaderLabel').setToolTip(tooltipLabel);

        //items repeater edit view
        each(sheet.get('items').value(), function (entryData, entryId) {
            sheet.get('items').find(entryId).find('weightUnit').text(weightUnit);
        })
    }
    setWeightUnit();
    sheet.get('unitWeight').on('update', setWeightUnit)

    const setSpeedUnit = function () {
        let distanceUnit = _(DistanceUnits.get(sheet.get("unitDistance").value()).nameShort)
        sheet.get('speedUnit').text(distanceUnit);
    }
    setSpeedUnit();
    sheet.get('unitDistance').on('update', function () {
        setSpeedUnit();
        setSpeedLabel(sheet)
    });

    sheet.get("jackOfAllTradesCheck").on("update", function () {
        setSkillLabels(sheet);
        setProficiencyIcons_skills(sheet);
    });

    sheet.get('remarkableAthleteCheck').on('update', function() {
        setSkillLabels(sheet);
        setProficiencyIcons_skills(sheet);
    });

    const displayIntimidationAttribute = function (){
        let useStrength = sheet.get("useStrengthForIntimidation").value();
        let attribute = useStrength ? "strength" : Skills.get("intimidation").attribute;
        sheet.get('intimidation_mod').text(Attributes.get(attribute).short);
        
        if(useStrength)
            sheet.get('intimidation_mod').addClass('text-primary').removeClass('opacity-50').setToolTip(_("Alternate rule active: using Strength instead of Charisma. See Settings Tab to deactivate."), 'right');
        else sheet.get('intimidation_mod').removeClass('text-primary').addClass('opacity-50').setToolTip("");
    };

    displayIntimidationAttribute();
    sheet.get("useStrengthForIntimidation").on("update", function () {
        setSkillLabel(sheet,"intimidation");
        displayIntimidationAttribute();
    });

    const toggleSendToChatButtonDisplay = function () {
        let shouldHide = sheet.get('deactivateSendingToChat').value()

        each(["weapons", "items", "spells", "features"], function (repeaterId) {
            let repeater = sheet.get(repeaterId);

            each(repeater.value(), function (entryData, entryId) {
                if (shouldHide)
                    repeater.find(entryId).find('buttonSendToChat').hide();
                else repeater.find(entryId).find('buttonSendToChat').show()
            })
        })
    }
    toggleSendToChatButtonDisplay();
    sheet.get("deactivateSendingToChat").on('update', toggleSendToChatButtonDisplay);

    const toggleEncumbranceRow = function () {
        let shoudleHide = sheet.get('hideEncumbrance').value();
        if (shoudleHide)
            sheet.get('encumbranceRow').hide();
        else sheet.get('encumbranceRow').show();
        setCarryingCapacityLabel(sheet);
    }
    toggleEncumbranceRow();
    sheet.get('hideEncumbrance').on('update', toggleEncumbranceRow);

    initTutorialPrompt(sheet);
    initDevLogPrompt(sheet);
};

const initToggleDetails = function (sheet) {
    let fieldList = {
        "main": ["weapons", "spells", "items", "features"],
        "monster": ["actionsRead_common", "actions"]
    };

    each(fieldList[sheet.id()], function (repeaterId) {
        let repeater = sheet.get(repeaterId);

        const toggleDetails = function (entry, shouldOpen) {
            if (shouldOpen) {
                entry.find('details').show();
                entry.find('openDetails').removeClass('d-inline').addClass('d-none');
                entry.find('closeDetails').addClass('d-inline').removeClass('d-none');
                entry.find('nameLabel').addClass('text-title');
            } else {
                entry.find('details').hide();
                entry.find('openDetails').addClass('d-inline').removeClass('d-none');
                entry.find('closeDetails').removeClass('d-inline').addClass('d-none');
                entry.find('nameLabel').removeClass('text-title');
            }
        };

        const toggleOne = function (target) {
            let entry = repeater.find(target.index())
            let shouldOpen = target.id().includes('open');
            toggleDetails(entry, shouldOpen);
        }

        const toggleAll = function (target) {
            let shouldOpen = target.id().includes('open');
            each(repeater.value(), function (entryData, entryId) {
                let entry = repeater.find(entryId);
                toggleDetails(entry, shouldOpen);
            })
        }

        repeater.on('click', 'openDetails', toggleOne);
        repeater.on('click', 'closeDetails', toggleOne);
        sheet.get('openAllDetails_' + repeaterId).on('click', toggleAll);
        sheet.get('closeAllDetails_' + repeaterId).on('click', toggleAll);
    });
};

const initToggleEdit = function (sheet, containerId) {
    const inputContainer = sheet.get(containerId + "EditContainer");
    let readContainer = "";
    try {
        readContainer = sheet.get(containerId + "ReadContainer");
    } catch (err) { };
    const editButton = sheet.get(containerId + "Edit");
    const doneButton = sheet.get(containerId + "Done");

    inputContainer.hide();
    editButton.on("click", function () {
        inputContainer.show();
        if (readContainer)
            readContainer.hide();
        editButton.hide();
        doneButton.show();
    })
    doneButton.on("click", function () {
        inputContainer.hide();
        if (readContainer)
            readContainer.show();
        editButton.show();
        doneButton.hide();
        if(containerId === "characterSummary"){
            setProficiencyBonus(sheet);
            setCharacterSummary(sheet);
            setHitDice(sheet);
        }
    })
}

const initTutorialPrompt = function (sheet) {
    let tutorialPromptId = sheet.id() === "main" ? "promptTutorial_Character" : "promptTutorial_Monster"
    sheet.get("tutorialBtn").on("click", function () {
        sheet.prompt(_("Tutorial"), tutorialPromptId, function () {
            return;
        })
    })
};

const initDevLogPrompt = function (sheet) {
    const showDevLog = function () {
        sheet.prompt(_("Development Log"), "promptDevLog", function () {
            return;
        })
    }
    sheet.get('devLogBtn').on('click', showDevLog)
};

const toggleHeaderRow = function (sheet, repeater) {
    let repeaterObject = repeater.value() || {};

    if (Object.keys(repeaterObject).length > 0)
        sheet.get(repeater.id() + 'Header_row').show();
    else sheet.get(repeater.id() + 'Header_row').hide();
};

//
// Modifiers & bonus getters
//

const getRaces = function (sheet) {
    let races = {};

    Races.each(function (race) {
        races[race.id] = _(race.name);
    })

    Object.assign(races, getCustomRaces(sheet));
    return races;
};

const getCustomRaces = function (sheet) {
    let customRacesRepeater = sheet.get("customRaces");

    let customRaces = [];
    each(customRacesRepeater.value(), function (entryData, entryId) {
        let customRaceName = entryData.customRaceName || null;
        if (customRaceName) {
            let newCustomRaceId = customRaceName.toLowerCase().replace(" ", "");
            customRaces[newCustomRaceId] = customRaceName;
        }
    });

    return customRaces;
};

const getClasses = function(sheet){
    let classes = {};

    Classes.each(function (classMetadata) {
        classes[classMetadata.id] = {
           "id": classMetadata.id,
           "name": _(classMetadata.name),
           "hitDie": classMetadata.hitDie,
           "spellcastingAbility": classMetadata.spellcastingAbility
        }
    })

    Object.assign(classes, getCustomClasses(sheet));
    return classes;
};

const getCustomClasses = function (sheet) {
    let customClassEntries = sheet.get("customClasses").value();
    let customClasses = {};

    each(customClassEntries, function (entryData, entryId) {
        let customClassName = entryData.customClassName || null;
        if (customClassName) {
            let newCustomClassId = trim(customClassName);
            customClasses[newCustomClassId] = {
                "id": newCustomClassId,
                "name": entryData.customClassName,
                "hitDie": entryData.hitDie || ":question:",
                "spellcastingAbility": entryData.spellcastingAbility || "strength"
            }
        }
    });

    return customClasses;
};

const getSelectedClasses = function(sheet){
    let selectedClasses = {};

    let baseClass = sheet.get('class').value() || "null";
    selectedClasses["baseClass"] = {
        id: baseClass,
        name: getClasses(sheet)[isValidClass(sheet, baseClass) ? baseClass : "null"].name,
        hitDie: getClasses(sheet)[isValidClass(sheet, baseClass) ? baseClass : "null"].hitDie,
        hitdiceCurrent: sheet.get("hitdiceCurrent").value(),
        level: sheet.get("level").value(),
        levelField: "level",
        classField: "class",
        hitDieClassField: "hitdiceClass",
        currentHitDiceField: "hitdiceCurrent",
        maxHitDiceField: "hitdiceMax",
        rollHitDiceButton: "hitDiceBtn",
        hitDieTypeField: "hitdiceType"
    };

    let firstMulticlass = sheet.get("multiClassChoice").value() || "null";
    let hasFirstMulticlass = sheet.get("multiClassLevel").value() || firstMulticlass != "null";
    if(hasFirstMulticlass){
        selectedClasses["firstMulticlass"] = {
            id: firstMulticlass,
            name: getClasses(sheet)[isValidClass(sheet, firstMulticlass) ? firstMulticlass : "null"].name,
            hitDie: getClasses(sheet)[isValidClass(sheet, firstMulticlass) ? firstMulticlass : "null"].hitDie,
            hitdiceCurrent: sheet.get("multiclassHitDiceCurrent").value(),
            level: sheet.get("multiClassLevel").value(),
            levelField: "multiClassLevel",
            classField: "multiClassChoice",
            hitDieClassField: "multiclassHitDiceClass",
            currentHitDiceField: "multiclassHitDiceCurrent",
            maxHitDiceField: "multiclassHitDiceMax",
            rollHitDiceButton: "multiclassHitDiceBtn",
            hitDieTypeField: "multiclassHitDieType"            
        }
    }

    let multiClassMore = sheet.get('multiClassMore').value() || {};
    let hasMoreMultiClass = Object.keys(multiClassMore).length;
    if(hasMoreMultiClass){
        each(multiClassMore, function(entryData, entryId){
            let classId = entryData["class"] || "null";
            let hitDiceMore = sheet.get('hitDiceMore').value();
            let hitDiceExistForThisClass = hitDiceMore.hasOwnProperty(entryId);
            let currentHitDice = hitDiceExistForThisClass ? hitDiceMore[entryId].hitdiceCurrent : entryData["level"];
            selectedClasses[entryId] = {
                id: classId,
                name: getClasses(sheet)[isValidClass(sheet, classId) ? classId : "null"].name,
                hitDie: getClasses(sheet)[isValidClass(sheet, classId) ? classId : "null"].hitDie,
                hitdiceCurrent: currentHitDice,
                level: entryData["level"] || 0,
                levelField: "level",
                classField: "class",
                hitDieClassField: "hitdiceClass",
                currentHitDiceField: "hitdiceCurrent",
                maxHitDiceField: "hitdiceMax",
                rollHitDiceButton: "hitDiceBtn",
                hitDieTypeField: "hitdiceType"
            };
        });
    }
    return selectedClasses;
};

const isValidClass = function(sheet, classId){
    return getClasses(sheet).hasOwnProperty(classId);
};

const isMulticlass = function(sheet) {
    let firstMulticlass = sheet.get('multiClassChoice').value() || "null";
    let firstMulticlassLvl = sheet.get("multiClassLevel").value();

    if(firstMulticlass && firstMulticlassLvl)
        return true;

    let multiClassMore = sheet.get("multiClassMore").value() || {};
    for (classIndex in multiClassMore)

    return firstMulticlassLvl > 0 || Object.keys(multiClassMore).length > 0;
};

const getLevel = function (sheet) {
    let level = 0;
    let selectedClasses = getSelectedClasses(sheet);

    for (classId in selectedClasses){
        level += parseInt(selectedClasses[classId].level || 0);
    }

    return Math.min(MAX_LEVEL, level);
};

const getHPCurrent = function (sheet) {
    return parseInt(sheet.get("hp").value());
};

const getHPMax = function (sheet) {
    return parseInt(sheet.get("hpmax").value())
};

const getAttributeScore = function (sheet, attribute) {
    let attributeScore = 10;

    if (isValidTableElement(Attributes, attribute)) {
        attributeScore = parseInt(sheet.get(attribute).rawValue() || 0)

        if (sheet.id() === "main") {
            attributeScore += getExtraBonus(sheet, attribute);
        }
    }
    return attributeScore;
};

const getAttributeModifier = function (sheet, attribute) {
    let attributeScore = getAttributeScore(sheet, attribute);
    return Math.floor((-10 + attributeScore) / 2);
};

const getSavingThrowBonus = function (sheet, attribute) {
    let saveBonus = 0;

    if (isValidTableElement(Attributes, attribute)) {
        if (sheet.id() === "main") {
            saveBonus += getAttributeModifier(sheet, attribute) + getExtraBonus(sheet, "save_" + attribute);

            let hasProficiency = sheet.get("save_" + attribute + "_prof").value();
            if(getExpertise(sheet, "save_" + attribute))
                saveBonus += 2*getProficiencyBonus(sheet);
            else if (hasProficiency)
                saveBonus += getProficiencyBonus(sheet);

        } else if (sheet.id() === "monster" || sheet.id() === "npc") {
            let attributeName = Attributes.get(attribute).name;
            let propertyId = getPropertyIndexFromName(sheet, "saving_throw", attributeName);
            if (propertyId) {
                let propertyValue = getPropertyValue(sheet, propertyId);
                saveBonus = parseInt(propertyValue.replace("+", ""));
            }
            else saveBonus = getAttributeModifier(sheet, attribute);
        }
    }
    return saveBonus;
};

const getSkillBonus = function (sheet, skill) {
    let skillBonus = 0;
    if (isValidTableElement(Skills, skill)) {
        let attribute = "";

        if(skill === "intimidation" && sheet.get('useStrengthForIntimidation').value())
            attribute = "strength";
        else attribute = Skills.get(skill).attribute;

        if (sheet.id() === "main") {

            skillBonus = getAttributeModifier(sheet, attribute) + getExtraBonus(sheet, skill);

            let hasProficiency = sheet.get(skill + "_prof").value();
            let hasRemarkableAthlete = sheet.get("remarkableAthleteCheck").value();
            let isPhysicalSkill = attribute == "strength" || attribute == "constitution" || attribute == "dexterity";
            let hasJackOfAllTrades = sheet.get("jackOfAllTradesCheck").value();

            if(getExpertise(sheet, skill))
                skillBonus += 2 * getProficiencyBonus(sheet);
            else if (hasProficiency)
                skillBonus += getProficiencyBonus(sheet);
            else if (hasRemarkableAthlete && isPhysicalSkill)
                skillBonus += Math.ceil(getProficiencyBonus(sheet) / 2);
            else if (hasJackOfAllTrades)
                skillBonus += Math.floor(getProficiencyBonus(sheet) / 2);

        } else if (sheet.id() === "monster" || sheet.id() === "npc") {
            let skillName = Skills.get(skill).name;
            let entryId = getPropertyIndexFromName(sheet, "skill", skillName);
            if (entryId) {
                let propertyValue = getPropertyValue(sheet, entryId);
                skillBonus = parseInt(propertyValue.replace("+", ""));
            }
            else skillBonus = getAttributeModifier(sheet, attribute);
        }
    }
    return skillBonus;
};

const getSpellSaveDC = function (sheet) {
    let spellSaveDC = 0;
    let spellSaveDCInput = parseInt(sheet.get("spellSaveDcInput").value());

    if (spellSaveDCInput > 0) {
        spellSaveDC = spellSaveDCInput
    } else {
        let spellcastingAttribute = sheet.get("spellcastingAbility").value();
        let spellcastingAttributeModifier = getAttributeModifier(sheet, spellcastingAttribute);

        spellSaveDC = BASE_SPELL_SAVE_DC + spellcastingAttributeModifier + getProficiencyBonus(sheet);
    }
    return spellSaveDC;
};

const getSpellAttackBonus = function (sheet) {
    let spellAttackBonus = 0;
    let spellAttackBonusInput = parseInt(sheet.get("spellAttackBonusInput").value());

    if (spellAttackBonusInput != 0) {
        spellAttackBonus = spellAttackBonusInput;
    } else {
        let spellcastingAttribute = sheet.get("spellcastingAbility").value();
        let spellcastingAttributeModifier = getAttributeModifier(sheet, spellcastingAttribute);

        spellAttackBonus = spellcastingAttributeModifier + getProficiencyBonus(sheet);
    }
    return spellAttackBonus;
};

const getInitiative = function (sheet) {
    let initiative = 0;
    let initiativeInput = parseInt(sheet.get("initInput").value());

    if (initiativeInput > 0) {
        initiative = initiativeInput;
    } else {
        initiative = getAttributeModifier(sheet, "dexterity");
        if (sheet.id() === 'main')
            initiative += parseInt(sheet.get("initBonus").value());
    }
    return initiative;
};

const getProficiencyBonus = function (sheet) {
    let proficiencyBonus = 0;
    let proficiencyBonusInput = parseInt(sheet.get("proficiencyInput").value())

    if (proficiencyBonusInput != 0) {
        proficiencyBonus = proficiencyBonusInput;
    } else {
        let level = getLevel(sheet);
        proficiencyBonus = Math.floor(2 + (level - 1) / 4);
    }
    return proficiencyBonus;
};

const getPassivePerception = function (sheet) {
    let passivePerception = 0;
    let passivePerceptionInput = 0;

    if (sheet.id() === "main") {
        passivePerceptionInput = parseInt(sheet.get("passivePerceptionInput").value());
    } else if (sheet.id() === "monster" || sheet.id() === "npc") {
        let propertyId = getPropertyIndexFromName(sheet, "sense", "Passive Perception");
        if (propertyId)
            passivePerceptionInput = parseInt(getPropertyValue(sheet, propertyId));
    }

    if (passivePerceptionInput > 0) {
        passivePerception = passivePerceptionInput;
    } else {
        passivePerception = 10 + getSkillBonus(sheet, "perception");
    }

    return passivePerception;
};

const getPassiveSkill = function (sheet, skill) {
    let passiveSkill = 0;
    let passiveSkillInput = 0;

    if (sheet.id() === "main") {
        let inputFieldID = PASSIVE_SKILLS[skill].fieldID + "Input";
        passiveSkillInput = parseInt(sheet.get(inputFieldID).value());
    } else if (sheet.id() === "monster" || sheet.id() === "npc") {
        let propertyName = PASSIVE_SKILLS[skill].propertyName;
        let propertyId = getPropertyIndexFromName(sheet, "sense", propertyName);
        if (propertyId)
            passiveSkillInput = parseInt(getPropertyValue(sheet, propertyId));
    }

    if (passiveSkillInput > 0) {
        passiveSkill = passiveSkillInput;
    } else {
        passiveSkill = 10 + getSkillBonus(sheet, skill);
    }

    return passiveSkill;
};

const getExtraBonus = function (sheet, field) {
    let bonusEntries = sheet.get("bonuses").value()
    let bonus = 0;

    each(bonusEntries, function (entryData, entryId) {
        if (entryData.bonusField == field) {
            if (entryData.extraBonus && entryData.extraBonus != 0) {
                bonus += parseInt(entryData.extraBonus);
            }
        }
    });
    return bonus;
};

const getExpertise = function (sheet, field){
    let bonusEntries = sheet.get("bonuses").value()
    let expertise = false;

    each(bonusEntries, function (entryData, entryId) {
        if (entryData.bonusField == field && entryData.expertise)
            expertise = true;
    });
    return expertise;
}

const getArmorClass = function (sheet) {
    let armorClass = BASE_ARMOR_CLASS;

    let armorClassInput = parseInt(sheet.get('acInput').value());
    if (armorClassInput > 0) {
        armorClass = armorClassInput;
    } else if (sheet.id() === "main") {
        let armorAC = parseInt(sheet.get("armorAc").value() || BASE_ARMOR_CLASS);
        let attributeOneModifier = getAttributeModifier(sheet, sheet.get("acAttributeOne").value());
        let attributeTwoModifier = getAttributeModifier(sheet, sheet.get("acAttributeTwo").value());
        let extraBonus = parseInt(sheet.get("acBonus").value() || 0);

        armorClass = armorAC + attributeOneModifier + attributeTwoModifier + extraBonus;
    } else if (sheet.id() === "monster" || sheet.id() === "npc"){
        armorClass = 10 + getAttributeModifier(sheet, "dexterity");
    }

    return armorClass;
};

const getSpeed = function (sheet) {
    let distanceUnit = "ft";
    let distanceUnitDisplay = _(DistanceUnits.get(distanceUnit).nameShort)
    let speed = 0;

    if (sheet.id() === "main") {
        let speedInput = parseInt(sheet.get("speedInput").value());
        distanceUnit = sheet.get("unitDistance").value();
        speed = speedInput > 0 ? speedInput : DistanceUnits.get(distanceUnit).baseSpeed;
    } else if (sheet.id() === "monster" || sheet.id() === "npc") {
        let speeds = [];

        let properties = sheet.get("properties").value();
        each(properties, function (entryData, entryId) {
            if (entryData["property"] == "speed") {
                let speedValue = entryData["propertyValue"] || "";
                if (speedValue) {
                    let hasNonNumericalCharacter = speedValue.toString().replace(" ", "").search(/\D/) >= 0
                    if (!hasNonNumericalCharacter)
                        speedValue += " " + distanceUnitDisplay;
                    let speedType = entryData["propertyName"] ? (entryData["propertyName"].toLowerCase() + " ") : "";
                    speeds.push(speedType + speedValue);
                }
            }
        })

        if (speeds.length)
            speed = speeds.join(", ");
        else speed = DistanceUnits.get(distanceUnit).baseSpeed + " " + distanceUnitDisplay;
    }
    return speed;
};

const getVisibility = function (sheet) {
    let visibility = sheet.get("diceVisibility").value();
    log(visibility)
    return visibility;
};

const getPropertyIndexFromName = function (sheet, property, propertyName) {
    let propertiesRepeaterId = PROPERTIES_IDS[sheet.id()].repeaterId;
    let properties = sheet.get(propertiesRepeaterId).value() || {};

    const match = function (entryId) {
        let entryData = properties[entryId];
        let propertyChoiceId = PROPERTIES_IDS[sheet.id()].choiceId;
        let propertyNameId = PROPERTIES_IDS[sheet.id()].nameId;
        let trimmedPropertyName = trim(entryData[propertyNameId]);

        if (entryData[propertyChoiceId] == property || entryData[propertyChoiceId] == MONSTER_EQUIVALENT_IDS[property]) {
            return (trimmedPropertyName == trim(propertyName) || trimmedPropertyName == trim(_(propertyName)));
        }
    }
    return Object.keys(properties).find(match);
};

const getPropertyValue = function (sheet, entryId) {
    let propertiesRepeaterId = PROPERTIES_IDS[sheet.id()].repeaterId;
    let properties = sheet.get(propertiesRepeaterId).value();
    let propertyValueId = PROPERTIES_IDS[sheet.id()].valueId;

    return properties[entryId][propertyValueId];
};

const getPropertyName = function (sheet, entryId) {
    let propertiesRepeaterId = PROPERTIES_IDS[sheet.id()].repeaterId;
    let Table = PROPERTIES_IDS[sheet.id()].table;
    let propertyChoiceId = PROPERTIES_IDS[sheet.id()].choiceId;

    let properties = sheet.get(propertiesRepeaterId).value();
    let selectedProperty = properties[entryId][propertyChoiceId] || "default";

    if(isValidTableElement(Table,selectedProperty))
        return _(Table.get(selectedProperty).name);
    else return selectedProperty;
};

const getPropertyTitle = function (sheet, entryId) {
    let propertiesRepeaterId = PROPERTIES_IDS[sheet.id()].repeaterId;
    let properties = sheet.get(propertiesRepeaterId).value();
    let propertyNameId = PROPERTIES_IDS[sheet.id()].nameId;

    return properties[entryId][propertyNameId];
};

const getSelectProperties = function (sheet, propertyType) {
    let propertiesRepeaterId = PROPERTIES_IDS[sheet.id()].repeaterId;
    let choiceId = PROPERTIES_IDS[sheet.id()].choiceId;

    let properties = sheet.get(propertiesRepeaterId).value();
    let selectedProperties = {};

    for (entry in properties) {
        if (properties[entry][choiceId] && (properties[entry][choiceId] == propertyType || (MONSTER_EQUIVALENT_IDS[propertyType] && properties[entry][choiceId] == MONSTER_EQUIVALENT_IDS[propertyType]))) {
            selectedProperties[entry] = properties[entry];
        }
    }
    return selectedProperties;
};

//
// region Dice
//

const rollAttribute = function (sheet, attribute) {
    let rollTitle = _(Attributes.get(attribute).name);
    createD20(sheet, function (d20) {
        let dicePool = d20.add(getAttributeModifier(sheet, attribute));
        Dice.roll(sheet, dicePool, rollTitle, getVisibility(sheet));
    })
};

const rollSave = function (sheet, attribute) {
    let rollTitle = _("%attribute saving throw").replace("%attribute", _(Attributes.get(attribute).name));
    createD20(sheet, function (d20) {
        let dicePool = d20.add(getSavingThrowBonus(sheet, attribute));
        Dice.roll(sheet, dicePool, rollTitle, getVisibility(sheet));
    })
};

const rollSkill = function (sheet, skill) {
    let rollTitle = _(Skills.get(skill).name)
    createD20(sheet, function (d20) {
        let dicePool = d20.add(getSkillBonus(sheet, skill));
        Dice.roll(sheet, dicePool, rollTitle, getVisibility(sheet));
    })
};

const rollInitiative = function (sheet) {
    let rollTitle = _("Initiative")
    createD20(sheet, function (d20) {
        let dicePool = d20.add(getInitiative(sheet)).tag("initiative");
        Dice.roll(sheet, dicePool, rollTitle, getVisibility(sheet));
    })
};

const rollDeathSave = function (target) {
    let sheet = target.sheet();
    let dicePool = Dice.create('1d20');
    let rollTitle = _("Death Save");

    const getLastCheckedBox = function(type) {
        let boxPrefix = "deathSave"+type;
        let counter = 0;
        for(let i = 1; i <= 3; i++){
            if (sheet.get(boxPrefix+i).value())
                counter = i;
        }

        return counter;
    }

    const incrementDeathSaves = function (type){
        let nextCheckbox = getLastCheckedBox(type) + 1;
        if(nextCheckbox <= 3)
            sheet.get("deathSave"+type+nextCheckbox).value(true);
    }

    const markResult = function(result) {
        if(result.total == 1) {
            incrementDeathSaves("Failure"); incrementDeathSaves("Failure");
        } else if (result.total < 10) {
            incrementDeathSaves("Failure");
        } else if (result.total < 20) {
            incrementDeathSaves("Success");
        } else {
            sheet.get("hp").value(1);
        }
    }

    let actionAlreadyClicked = false;
    let actions = {};
    actions[_("Mark Result")] = function (result) {
        if (!actionAlreadyClicked) {
            actionAlreadyClicked = true;
            markResult(result);
        }
    };

    Dice.roll(sheet, dicePool, rollTitle, getVisibility(sheet), actions);   
};

const rollHitDice = function (target, sheet) {
    let promptView = target.sheet();
    let hitDieRoll = promptView.get("hitDiceMore").find(target.index()).find("hitdiceType").value();
    let hitDieClass = promptView.get("hitDiceMore").find(target.index()).find("hitdiceClass").value();
    let dicePool = Dice.create(hitDieRoll);
    let rollTitle = _(hitDieClass) + " - " + _("Hit Die");

    const applyHealing = function(result){
        let currentHP = getHPCurrent(sheet);
        let maxHP = getHPMax(sheet);
        let newHP = Math.min(maxHP, currentHP + result.total);

        sheet.get("hp").value(newHP);
    }

    const incrementHitDice = function(){
        let currentHitDiceField = promptView.get("hitDiceMore").find(target.index()).find("hitdiceCurrent");
        let currentHitDice =  currentHitDiceField.value();
        currentHitDiceField.value(currentHitDice - 1);
    }

    let healActionAlreadyClicked = false;
    let actions = {};
    actions[_("Heal")] = function (result) {
        if (!healActionAlreadyClicked) {
            healActionAlreadyClicked = true;
            applyHealing(result);
            incrementHitDice();
        }
    };

    Dice.roll(sheet, dicePool, rollTitle, getVisibility(sheet), actions);
};

const rollAttack = function (sheet, attackSourceName, attackBonus, damageRoll) {
    attackBonus = attackBonus.toString().replace(/\s+/g, '');//replace spaces with empty string = removes spaces
    let rollTitle = attackSourceName;
    let actions = {};
    if (damageRoll && damageRoll !== "0") {
        actions[_("Roll Damage")] = function (attackResult) {
            rollDamages(sheet, attackSourceName, damageRoll, isCritical(attackResult));
        }
    }
    createD20(sheet, function (d20) {
        let dicePool = d20.add(attackBonus);
        Dice.roll(sheet, dicePool, rollTitle, getVisibility(sheet), actions);
    })
};

const rollDamages = function (sheet, damageSourceName, damageRoll, attackWasCritical) {
    damageRoll = damageRoll.replace(/\s+/g, ''); //replace spaces with empty string = removes spaces

    let rollTitle = _(damageRoll.includes("heal") ? "Heal" : "Damage") + _(":") + " %damageSourceName"
    rollTitle = _(rollTitle).replace("%damageSourceName", damageSourceName);
    let dicePool = Dice.create(damageRoll)

    if (attackWasCritical) {
        dicePool = dicePool.mul(2);
    }

    Dice.roll(sheet, dicePool, rollTitle, getVisibility(sheet));
};

const createD20 = function (sheet, callback) {

    let throwType = sheet.get("diceThrow").value();

    let toggleTypeValue = function (target) {
        target.sheet().get(throwType).removeClass("active").removeClass('shadow-sm');
        throwType = target.id();
        target.sheet().get(target.id()).addClass("active").addClass('shadow-sm');
    }

    let createRoll = function (throwType) {
        if (throwType === "advantage") {
            return Dice.create("2d20").keeph();
        } else if (throwType === "disadvantage") {
            return Dice.create("2d20").keepl();
        } else {
            return Dice.create("1d20");
        }
    }

    if (throwType == "ask") {
        throwType = 'normal';

        sheet.prompt(_('Select the type of roll'), "promptAskAdvantage",
            function (result) {
                callback(createRoll(throwType));
            }, function (promptView) {
                promptView.get('advantage').on('click', toggleTypeValue);
                promptView.get('disadvantage').on('click', toggleTypeValue);
                promptView.get('normal').on('click', toggleTypeValue);
            }
        );
    }
    else callback(createRoll(throwType));
};

const isCritical = function (result) {
    let flat = [];
    extractD20(flat, result);

    for (var i = 0; i < flat.length; i++) {
        if (flat[i].dimension == 20) {
            if (flat[i].values[0] == 20) {
                return true;
            }

            return false;
        }
    }
};

const extractD20 = function (flat, result) {
    for (var i = 0; i < result.children.length; i++) {
        flat.push(result.children[i]);
        extractD20(flat, result.children[i]);
    }
};

// endregion
//
// General functions
//

const isValidTableElement = function (table, elementToTest) {
    let isValid = false;

    table.each(function (tableElement) {
        if (tableElement.id == elementToTest)
            isValid = true;
    });

    return isValid;
};

const getTableElementFromName = function (table, name) {
    let id = "";

    table.each(function (element) {
        if (trim(element.name) == trim(name) || trim(_(element.name)) == trim(name)) {
            id = element.id;
        }
    })
    return id;
};

const errorPrompt = function (sheet, errorType) {
    sheet.prompt(_("Error"), "promptError", function () { }, function (promptView) {
        let errorDescription = ErrorMessages.get(errorType).errorDescription
        promptView.get("errorDescription").text(errorDescription);
    })
};

const trim = function (string) {
    if(string){
        return string.trim().toLowerCase().replace(" ", "");
    } else return "";
};

const sortObjectByValues = function (obj, sortFunction) {
    const sortedKeys = Object.keys(obj).sort(sortFunction);

    const sortedObj = sortedKeys.reduce(function (result, key) {
        result[key] = obj[key];
        return result;
    }, {});

    return sortedObj;
};

const objectsEqual = function (object1, object2) {
    let objectsEqual = false;

    if (typeof object1 == 'object' && typeof object2 == 'object') {
        objectsEqual = true;
        let keys1 = Object.keys(object1);
        let keys2 = Object.keys(object2);


        if (keys1.length !== keys2.length)
            objectsEqual = false;

        else {
            each(object1, function (item, index) {
                if (item !== object2[index])
                    objectsEqual = false;
            });
        }
    }
    return objectsEqual;
};

const getSign = function (value) {
    let sign = "";
    if(value > 0 || value == 0 && arguments.length === 1)
        sign = "+";
    return sign;
};

const initRepeater = function (repeaterComponent, initCallback, updateCallback) { // [called in initRollMemo and initDisciplines] this function initializes a repeater: initCallback is the function to execute at sheet inititialization for each item in the repeater, and updateCallback is the function to execute whenever an item within the repeater is updated 

    const initOne = function (index) { // this function is called later on for each item in the repeater
        initCallback(repeaterComponent, index); // this happens when the sheet is first initialized
        repeaterComponent.find(index).on('update', function(target) {
            if(target.value()) // if it's not a deletion
                updateCallback(repeaterComponent, target.index());
        });
    };

    let currentIndexes = []; // this array stores the list of items in the repeater, to check later on whether the updated item is new or not
    if (repeaterComponent.value())
        currentIndexes = Object.keys(repeaterComponent.value()); // if the repeater is not empty (new sheet for example), store all the item keys in currentItems for later reference

    for (index in repeaterComponent.value()) { // at sheet initialization, run through every item in the repeater, and initialize it
        initOne(index);
    }

    repeaterComponent.on('update', function(){ // whenever the repeater is updated
        for (index in repeaterComponent.value()) { // run through every item in the repeater
            if (!currentIndexes.includes(index)) // if it's a newly created item
                initOne(index); // initialize it
        }
        if (repeaterComponent.value()) // refresh currentItems with the new list of items
            currentIndexes = Object.keys(repeaterComponent.value());
        else currentIndexes = [];
    });
};