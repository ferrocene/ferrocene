.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers


Traceability with ISO 26262 Requirements
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. csv-table:: ISO 26262 traceability
   :header:  ISO 26262 clause, Clause Description, Document chapter(s)
   :delim: !

   11.4.1! If the safety lifecycle incorporates the use of a software tool for the development of a system, or its hardware or software elements, such that activities or tasks required by ISO 26262 series of standards rely on the correct functioning of a software tool, and where the relevant outputs of that tool are not examined or verified for the applicable process step(s), such software tools shall comply with the requirements of this clause.! See the traceability below
   11.4.2!If the confidence level evaluation or qualification of a software tool is performed independently from the development of a particular safety-related item or element, the validity of this predetermined Tool Confidence Level or qualification shall be verified prior to the software tool being used for the development of a particular safety-related item or element.! N/A
   11.4.3!When using a software tool, it shall be ensured that its usage, its determined environmental and functional constraints and its general operating conditions comply with its evaluation criteria or its qualification.! :doc:`Safety Manual - Usage <safety-manual:usage>`
   11.4.4.1!The usage of a software tool shall be planned, including the determination of:!See sub items below
   11.4.4.1.a!the identification and version number of the software tool;! :doc:`Safety Manual - Environment <safety-manual:environment>`
   11.4.4.1.b!the configuration of the software tool;! :doc:`Safety Manual - Tool options <safety-manual:options>`
   11.4.4.1.c!the use cases of the software tool;! :doc:`Evaluation Report - Use cases <evaluation-report:use-cases>`
   11.4.4.1.d!the environment in which the software tool is executed,! :doc:`Safety Manual - Environment <safety-manual:environment>`
   11.4.4.1.e!the maximum ASIL of all the safety requirements, allocated to the item or the element that can directly be violated, if the software tool is malfunctioning and producing corresponding erroneous output; and! :doc:`Qualification Report - Acceptability statement <qualification-report:statement>`
   11.4.4.1.f!the methods to qualify the software tool, if required, based on the determined level of confidence and ASIL.! :doc:`Evaluation Report - Qualification Method <evaluation-report:method>`
   11.4.4.2!To ensure the proper evaluation or usage of the software tool, the following information shall be available:! See sub items below
   11.4.4.2.a!a description of the features, functions and technical properties of the software tool;! :doc:`Qualification Plan - Ferrocene details <qualification-plan:details>`
   11.4.4.2.b!the user manual or other usage guides, if applicable;! :doc:`Safety Manual - Associated documents <safety-manual:references>`
   11.4.4.2.c!a description of the environment required for its operation,! :doc:`Safety Manual - Environment <safety-manual:environment>`
   11.4.4.2.d!a description of the expected behaviour of the software tool under anomalous operating conditions, if applicable;! :doc:`Safety Manual - Degraded environment <safety-manual:degraded-environment>`
   11.4.4.2.e!a description of known software tool malfunctions and the appropriate safeguards, avoidance or workaround measures, if applicable; and! :doc:`Safety Manual - Known problems <safety-manual:known-problems>`
   11.4.4.2.f!the measures for the prevention or detection of malfunctions and the corresponding erroneous output of the software tool identified during the determination of the required level of confidence for this software tool.! :doc:`Safety Manual - Known Problems <safety-manual:known-problems>`
   11.4.5.1!The description of the usage of a software tool shall contain the following information:! See sub items below
   11.4.5.1.a!the intended purpose;! :doc:`Evaluation Report - Use cases <evaluation-report:use-cases>`
   11.4.5.1.b!the inputs and expected outputs; and! :doc:`Evaluation Report - Use cases <evaluation-report:use-cases>`
   11.4.5.1.c!the usage procedure, environmental and functional constraints, if applicable.! :doc:`Evaluation Report - Use cases <evaluation-report:use-cases>`
   11.4.5.2!The intended usage of the software tool shall be analysed and evaluated to determine:!See sub items below
   11.4.5.2.a!the possibility that a malfunction of a particular software tool can introduce or fail to detect errors in a safety-related item or element being developed. This is expressed by the classes of Tool Impact (TI):! See sub items below
   11.4.5.2.a.1!TI1 shall be selected when there is an argument that there is no such possibility;! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`
   11.4.5.2.a.2!TI2 shall be selected in all other cases;! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`
   11.4.5.2.b!the confidence in measures that prevent the software tool from malfunctioning and producing corresponding erroneous output, or in measures that detect that the software tool has malfunctioned and has produced corresponding erroneous output. This is expressed by the classes of Tool error Detection (TD):! See sub items below
   11.4.5.2.b.1!TD1 shall be selected if there is a high degree of confidence that a malfunction and its corresponding erroneous output will be prevented or detected! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`
   11.4.5.2.b.2!TD2 shall be selected if there is a medium degree of confidence that a malfunction and its corresponding erroneous output will be prevented or detected;! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`
   11.4.5.2.b.3!TD3 shall be selected in all other cases.! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`
   11.4.5.3!If the correct selection of TI or TD is unclear or doubtful, TI and TD should be estimated conservatively.! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`
   11.4.5.4!Based on the values determined for the classes of TI and TD (in accordance with 11.4.5.2 or 11.4.5.3), the required software Tool Confidence Level shall be determined according to Table 3.! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`
   11.4.6.1!For the qualification of software tools classified at TCL3, the methods listed in Table 4 shall be applied. For the qualification of software tools classified at TCL2, the methods listed in Table 5 shall be applied. A software tool classified at TCL1 needs no qualification methods.! :doc:`Evaluation Report - Qualification method <evaluation-report:method>`
   11.4.6.2!The qualification of the software tool shall be documented including the following:! See sub items below
   11.4.6.2.a!the unique identification and version number of the software tool;! :doc:`Safety Manual - Environment <safety-manual:environment>`
   11.4.6.2.b!the maximum Tool Confidence Level for which the software tool is classified together with a reference to its evaluation analysis;! :doc:`Qualification Report - Acceptability statement <qualification-report:statement>`
   11.4.6.2.c!for the considered use cases the pre-determined maximum ASIL, or specific ASIL, of any safety requirement which might directly be violated if the software tool is malfunctioning and produces corresponding erroneous output;! :doc:`Qualification Report - Acceptability Statement <qualification-report:statement>`
   11.4.6.2.d!the configuration and environment for which the software tool is qualified;! :doc:`Safety Manual - Environment <safety-manual:environment>`
   11.4.6.2.e!the person or organization who carried out the qualification;! :doc:`Qualification Plan - Qualification organization <organization>`
   11.4.6.2.f!the methods applied for its qualification in accordance with 11.4.6.1;! :doc:`Evaluation Report - Qualification method <evaluation-report:method>`
   11.4.6.2.g!the results of the measures applied to qualify the software tool; and! :doc:`Qualification Report - Test results <qualification-report:tests>`
   11.4.6.2.h!the usage constraints and malfunctions identified during the qualification, if applicable.! :doc:`Qualification Report - Test results <qualification-report:tests>`
   11.4.7! Increased confidence from use! N/A
   11.4.8.1!If the method "Evaluation of the tool development process" in accordance with Table 4 or Table 5 is applied for the qualification of a software tool, the qualification shall comply with the requirements of this sub-clause.! :doc:`Evaluation Report - Qualification method <evaluation-report:method>`
   11.4.8.2!The development process applied for the development of the software tool shall comply with an appropriate standard.! :doc:`Qualification Plan - Development process <qualification-plan:development>`
   11.4.8.3!The evaluation of the development process applied for the development of the software tool shall be based on an appropriate national or international standard and provide evidence that a suitable software development process has been applied.! :doc:`Qualification Plan - Development process <qualification-plan:development>`
   11.4.9!Validation of the software tool! :doc:`Qualification Plan - Validation process <qualification-plan:validation>`
   11.4.9.1!If the method "Validation of the software tool" according to Table 4 or Table 5 is applied for the qualification of a software tool, the qualification shall comply with requirements of this sub-clause.! See items 11.4.9.2 below
   11.4.9.2!The validation of the software tool shall meet the following criteria:! See sub items below
   11.4.9.2.a!the validation measures shall provide evidence that the software tool complies with specified requirements to its purpose as specified in the classification;! :doc:`Qualification Plan - Validation process <qualification-plan:validation>`
   11.4.9.2.b!the malfunctions and their corresponding erroneous outputs of the software tool occurring during validation shall be analysed together with information on their possible consequences and with measures to avoid or detect them; and! :doc:`Qualification Report - Test results <qualification-report:tests>`
   11.4.9.2.c!the reaction of the software tool to anomalous operating conditions shall be examined;! :doc:`Safety Manual - Degraded environment <safety-manual:degraded-environment>`

.. end of table

Traceability with IEC 61508 Requirements
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. csv-table:: IEC 61508 traceability
   :header: IEC 61508 clause, Clause Description, Document chapter(s)
   :delim: !

   7.4.4.1!A software on-line support tool shall be considered to be a software element of the safety related system.! N/A
   7.4.4.2!Software off-line support tools shall be selected as a coherent part of the software development activities.! N/A
   7.4.4.3!The selection of the off-line support tools shall be justified! This qualification is the justification
   7.4.4.4!All off-line support tools in classes T2 and T3 shall have a specification or product documentation which clearly defines the behaviour of the tool and any instructions or constraints on its use.! :doc:`Safety Manual - Usage <safety-manual:usage>`, :doc:`Safety Manual - Degraded environment <safety-manual:degraded-environment>`, and :doc:`Evaluation Report - Use cases <evaluation-report:use-cases>`
   7.4.4.5!An assessment shall be carried out for offline support tools in classes T2 and T3 to determine the level of reliance placed on the tools, and the potential failure mechanisms of the tools that may affect the executable software. Where such failure mechanisms are identified, appropriate mitigation measures shall be taken.! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`
   7.4.4.6!For each tool in class T3, evidence shall be available that the tool conforms to its specification or documentation. Evidence may be based on a suitable combination of history of successful use in similar environments and for similar applications (within the organization or other organizations), and of tool validation as specified in 7.4.4.7.! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`, and :doc:`Qualification Report - Test results <qualification-report:tests>`
   7.4.4.7!The results of tool validation shall be documented covering the following results:! See items below
   7.4.4.7.a! a chronological record of the validation activities;! :doc:`Qualification Plan - Validation process <qualification-plan:validation>`
   7.4.4.7.b! the version of the tool product manual being used;! :doc:`Document List <document-list:index>`
   7.4.4.7.c! the tool functions being validated;! :doc:`Evaluation Report - Use cases <evaluation-report:use-cases>`
   7.4.4.7.d! tools and equipment used; ! :doc:`Evaluation Report - Qualification method <evaluation-report:method>`
   7.4.4.7.e! the results of the validation activity; the documented results of validation shall state either that the software has passed the validation or the reasons for its failure;! :doc:`Qualification Report - Test results <qualification-report:tests>`
   7.4.4.7.f! test cases and their results for subsequent analysis;! :doc:`Qualification Report - Test results <qualification-report:tests>`
   7.4.4.7.g! discrepancies between expected and actual results.! :doc:`Qualification Report - Test results <qualification-report:tests>`
   7.4.4.8! Where the conformance evidence of 7.4.4.6 is unavailable, there shall be effective measures to control failures of the executable safety related system that result from faults that are attributable to the tool.! N/A
   7.4.4.9!The compatibility of the tools of an integrated toolset shall be verified.! N/A
   7.4.4.10!To the extent required by the safety integrity level, the software or design representation (including a programming language) selected shall:! See items below
   7.4.4.10.a! have a translator which has been assessed for fitness for purpose including, where appropriate, assessment against the international or national standards;! :doc:`Evaluation Report - Tool analysis <evaluation-report:tool-analysis>`
   7.4.4.10.b! use only defined language features;! :doc:`Safety Manual - Tool Options <safety-manual:options>`
   7.4.4.10.c! match the characteristics of the application;! :doc:`Evaluation Report - Qualification method <evaluation-report:method>`
   7.4.4.10.d! contain features that facilitate the detection of design or programming mistakes;! :doc:`Safety Manual - Tool options <safety-manual:options>`
   7.4.4.10.e! support features that match the design method.! :doc:`Safety Manual - Tool options <safety-manual:options>`
   7.4.4.11!Where 7.4.4.10 cannot be fully satisfied, the fitness for purpose of the language, and any additional measures which address any identified shortcomings of the language shall be justified.! N/A
   7.4.4.12!Programming languages for the development of all safety-related software shall be used according to a suitable programming language coding standard.! :doc:`Qualification Plan - Development process <qualification-plan:development>`
   7.4.4.13!A programming language coding standard shall specify good programming practice, proscribe unsafe language features (for example, undefined language features, unstructured designs, etc.), promote code understandability, facilitate verification and testing, and specify procedures for source code documentation. Where practicable, the following information shall be contained in the source code:! See items below
   7.4.4.13.a! legal entity (for example company, author(s), etc.);! N/A
   7.4.4.13.b! description;! N/A
   7.4.4.13.c! inputs and outputs;! N/A
   7.4.4.13.d! configuration management history.! N/A
   7.4.4.14!Where automatic code generation or similar automatic translation takes place, the suitability of the automatic translator for safety-related system development shall be assessed at the point in the development lifecycle where development support tools are selected.! This qualification
   7.4.4.15!Where off-line support tools of classes T2 and T3 generate items in the configuration baseline, configuration management shall ensure that information on the tools is recorded in the configuration baseline. This includes in particular:! See items below
   7.4.4.15.a! the identification of the tool and its version;! N/A
   7.4.4.15.b! the identification of the configuration baseline items for which the tool version has been used;! N/A
   7.4.4.15.c! the way the tool was used (including the tool parameters, options and scripts selected) for each configuration baseline item.! N/A
   7.4.4.16!Configuration management shall ensure that for tools in classes T2 and T3, only qualified versions are used.! N/A
   7.4.4.17!Configuration management shall ensure that only tools compatible with each other and with the safety-related system are used.! N/A
   7.4.4.18!Each new version of off-line support tool shall be qualified. This qualification may rely on evidence provided for an earlier version if sufficient evidence is provided that:! See items below
   7.4.4.18.a! the functional differences (if any) will not affect tool compatibility with the rest of the toolset; and! N/A
   7.4.4.18.b! the new version is unlikely to contain significant new, unknown faults.! N/A
   7.4.4.19!Depending on the nature of the software development, responsibility for conformance with 7.4.4 can rest with multiple parties. The division of responsibility shall be documented during safety planning (see Clause 6 of IEC 61508-1).! N/A

.. end of table

